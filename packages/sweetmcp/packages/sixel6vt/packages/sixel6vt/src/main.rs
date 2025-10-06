// sixel6vt - Terminal with browser screenshots as sixel graphics
// Uses rioterm's Application with wrapper to intercept window creation

mod browser;
mod renderer;

use anyhow::Result;
use rio_backend::config::Config;
use rio_backend::event::{EventPayload, EventProxy, RioEvent, RioEventType};
use rio_window::application::ApplicationHandler;
use rio_window::event::WindowEvent;
use rio_window::event_loop::{ActiveEventLoop, EventLoop};
use rio_window::window::WindowId;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc as tokio_mpsc;
use tracing_subscriber;

/// Wrapper around rioterm::Application that intercepts window creation
/// to send the window ID to worker threads via channel
struct ApplicationWrapper<'a> {
    app: rioterm::Application<'a>,
    window_tx: Option<Sender<WindowId>>,
}

impl<'a> ApplicationWrapper<'a> {
    fn new(app: rioterm::Application<'a>, window_tx: Sender<WindowId>) -> Self {
        Self {
            app,
            window_tx: Some(window_tx),
        }
    }
}

impl ApplicationHandler<EventPayload> for ApplicationWrapper<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("resumed() called");
        self.app.resumed(event_loop);
    }

    fn new_events(
        &mut self,
        event_loop: &ActiveEventLoop,
        cause: rio_window::event::StartCause,
    ) {
        tracing::info!("new_events called with cause: {:?}", cause);

        // Delegate to inner application
        self.app.new_events(event_loop, cause);

        tracing::info!("After delegation, routes count: {}", self.app.router.routes.len());

        // After window creation, send the window ID once
        if let Some(tx) = self.window_tx.take() {
            if let Some(&window_id) = self.app.router.routes.keys().next() {
                tracing::info!("Window created with ID: {:?}", window_id);
                if let Err(e) = tx.send(window_id) {
                    tracing::error!("Failed to send window_id: {:?}", e);
                }
            } else {
                tracing::info!("No routes found yet, putting tx back");
                self.window_tx = Some(tx);
            }
        } else {
            tracing::info!("window_tx already sent or consumed");
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: EventPayload) {
        self.app.user_event(event_loop, event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.app.window_event(event_loop, window_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Check routes count but only log once when window appears
        if self.window_tx.is_some() && !self.app.router.routes.is_empty() {
            tracing::info!("about_to_wait: {} routes exist, attempting to send window_id", self.app.router.routes.len());
            if let Some(tx) = self.window_tx.take() {
                if let Some(&window_id) = self.app.router.routes.keys().next() {
                    tracing::info!("Sending window_id from about_to_wait: {:?}", window_id);
                    if let Err(e) = tx.send(window_id) {
                        tracing::error!("Failed to send window_id: {:?}", e);
                    }
                }
            }
        }
        self.app.about_to_wait(event_loop);
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.app.exiting(event_loop);
    }
}

fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("sixel6vt=debug,rioterm=info,rio_backend=info")
        .init();

    tracing::info!("Starting sixel6vt using rioterm");

    // Create channel for window ID communication
    let (window_tx, window_rx) = mpsc::channel::<WindowId>();

    // Create tokio runtime for browser operations
    let runtime = Runtime::new()?;

    // Load Rio configuration
    let config = Config::try_load().unwrap_or_else(|_| {
        tracing::warn!("Failed to load Rio config, using defaults");
        Config::default()
    });

    // Create event loop
    let event_loop = EventLoop::<EventPayload>::with_user_event().build()?;

    // Get event proxy BEFORE creating Application
    let event_proxy = EventProxy::new(event_loop.create_proxy());

    // Create rioterm Application
    let app = rioterm::Application::new(config, None, &event_loop);

    // Wrap application to intercept window creation
    let mut app_wrapper = ApplicationWrapper::new(app, window_tx);

    // Spawn worker thread for browser screenshot capture
    let proxy_clone = event_proxy.clone();
    std::thread::spawn(move || {
        runtime.block_on(async {
            // Wait for window ID from main thread
            tracing::info!("Waiting for window to be created...");
            let window_id = match window_rx.recv() {
                Ok(id) => {
                    tracing::info!("Window ID received: {:?}", id);
                    id
                }
                Err(e) => {
                    tracing::error!("Failed to receive window ID: {}", e);
                    return;
                }
            };

            // Additional wait for window initialization
            tokio::time::sleep(Duration::from_secs(2)).await;

            tracing::info!("Starting browser screenshot capture...");

            // Create channel for screenshots
            let (tx, mut rx) = tokio_mpsc::channel(1);

            // Capture screenshot
            let url = "https://github.com/trending";
            tokio::spawn(async move {
                if let Err(e) = browser::stream_screenshots(url, tx).await {
                    tracing::error!("Failed to stream screenshots: {}", e);
                }
            });

            // Wait for screenshot
            if let Some(snapshot) = rx.recv().await {
                tracing::info!("Screenshot received ({}), encoding to sixel...", snapshot.title);

                // Encode to sixel
                let sixel = renderer::encode_sixel(&snapshot.image);
                tracing::info!("Sixel encoded ({} bytes), sending to terminal...", sixel.len());

                // Send PtyWrite event with REAL window ID
                proxy_clone.send_event(
                    RioEventType::Rio(RioEvent::PtyWrite(format!("\r\n{}\r\n", sixel))),
                    window_id,
                );
                tracing::info!("Sixel event sent to window {:?}!", window_id);
                tracing::info!("Screenshot rendered. Terminal remains open for interaction.");
            }

            // Keep worker thread alive - terminal will continue running until user closes window
            // The tokio runtime stays alive here to prevent cleanup issues
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        });
    });

    // Run the wrapped application
    tracing::info!("Running rioterm application...");
    event_loop.run_app(&mut app_wrapper)?;

    Ok(())
}
