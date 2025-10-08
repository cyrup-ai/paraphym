// sixel6vt - Terminal with browser screenshots as sixel graphics
// Uses rioterm's Application with wrapper to intercept window creation

mod browser;
mod renderer;

use anyhow::Result;
use rio_backend::ansi::graphics::UpdateQueues;
use rio_backend::config::Config;
use rio_backend::event::{EventPayload, EventProxy, RioEvent, RioEventType};
use rio_backend::sugarloaf::{ColorType, GraphicData, GraphicId};
use rio_window::application::ApplicationHandler;
use rio_window::event::WindowEvent;
use rio_window::event_loop::{ActiveEventLoop, EventLoop};
use rio_window::window::WindowId;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::oneshot;
use tracing_subscriber;

/// Wrapper around rioterm::Application that intercepts window creation
/// to send the window ID to async tasks via oneshot channel
struct ApplicationWrapper<'a> {
    app: rioterm::Application<'a>,
    window_tx: Option<oneshot::Sender<WindowId>>,
}

impl<'a> ApplicationWrapper<'a> {
    fn new(app: rioterm::Application<'a>, window_tx: oneshot::Sender<WindowId>) -> Self {
        Self {
            app,
            window_tx: Some(window_tx),
        }
    }
}

impl ApplicationHandler<EventPayload> for ApplicationWrapper<'_> {
    #[inline]
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("resumed() called");
        self.app.resumed(event_loop);
    }

    #[inline]
    fn new_events(
        &mut self,
        event_loop: &ActiveEventLoop,
        cause: rio_window::event::StartCause,
    ) {
        tracing::info!("new_events called with cause: {:?}", cause);

        // Delegate to inner application
        self.app.new_events(event_loop, cause);

        tracing::info!("After delegation, routes count: {}", self.app.router.routes.len());

        // After window creation, send the window ID once via oneshot
        tracing::debug!("window_tx is_some: {}, routes count: {}", 
            self.window_tx.is_some(), self.app.router.routes.len());
        
        if let Some(tx) = self.window_tx.take() {
            if let Some(&window_id) = self.app.router.routes.keys().next() {
                tracing::info!("✓ Sending window_id: {:?}", window_id);
                if let Err(_) = tx.send(window_id) {
                    tracing::warn!("Failed to send window_id: receiver dropped");
                } else {
                    tracing::info!("✓ Window created, sent ID via oneshot channel");
                }
            } else {
                tracing::debug!("No routes yet (count={}), putting tx back", self.app.router.routes.len());
                self.window_tx = Some(tx);
            }
        } else if !self.app.router.routes.is_empty() {
            tracing::warn!("⚠ Routes exist but window_tx already consumed!");
        }
    }

    #[inline]
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: EventPayload) {
        self.app.user_event(event_loop, event);
    }

    #[inline]
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.app.window_event(event_loop, window_id, event);
    }

    #[inline]
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.app.about_to_wait(event_loop);
    }

    #[inline]
    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.app.exiting(event_loop);
    }
}

/// Atomic counter for generating unique GraphicIds as fallback
static GRAPHIC_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Convert image::RgbImage to Rio's GraphicData format (RGBA)
/// This matches the format Rio's sixel parser produces at crosswords/mod.rs:2959
fn rgb_image_to_graphic_data(rgb_img: &image::RgbImage) -> GraphicData {
    let width = rgb_img.width() as usize;
    let height = rgb_img.height() as usize;
    
    // Convert RGB (3 bytes/pixel) to RGBA (4 bytes/pixel) using iterator chains
    // Rio's graphics pipeline expects RGBA (verified in graphics.rs:190)
    let rgba_pixels: Vec<u8> = rgb_img
        .pixels()
        .flat_map(|p| [p[0], p[1], p[2], 255])
        .collect();
    
    // Create unique ID using timestamp (same approach as Rio's next_id())
    // Fallback to atomic counter if system time is unavailable
    let id = GraphicId(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or_else(|_| {
                // System time before UNIX_EPOCH - use atomic counter with process ID
                let pid = std::process::id() as u64;
                let counter = GRAPHIC_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                (pid << 32) | counter
            })
    );
    
    GraphicData {
        id,
        width,
        height,
        color_type: ColorType::Rgba,
        pixels: rgba_pixels,
        is_opaque: true,
        resize: None,
    }
}

fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("sixel6vt=debug,rioterm=info,rio_backend=info")
        .init();

    tracing::info!("Starting sixel6vt using rioterm");

    // Create tokio runtime (spawns background worker threads)
    let runtime = Runtime::new()?;

    // Create oneshot channel for window_id
    let (window_tx, window_rx) = oneshot::channel::<WindowId>();

    // Load Rio configuration
    let config = Config::try_load().unwrap_or_else(|_| {
        tracing::warn!("Failed to load Rio config, using defaults");
        Config::default()
    });

    // Create event loop
    let event_loop = EventLoop::<EventPayload>::with_user_event().build()?;

    // Get event proxy BEFORE creating Application
    let event_proxy = EventProxy::new(event_loop.create_proxy());

    // Clone event proxy for async task
    let proxy_clone = event_proxy.clone();

    // Spawn async task on runtime's worker threads
    runtime.spawn(async move {
        // Await window_id from oneshot channel (no blocking!)
        let window_id = match window_rx.await {
            Ok(id) => {
                tracing::info!("✓ Async task received window_id: {:?}", id);
                id
            }
            Err(e) => {
                tracing::error!("Failed to receive window_id: {}", e);
                return;
            }
        };

        // Wait for window initialization
        tokio::time::sleep(Duration::from_secs(2)).await;

        tracing::info!("Starting browser screenshot capture...");

        // Create channel for screenshots
        let (tx, mut rx) = tokio_mpsc::channel(1);

        // Spawn browser screenshot task
        let url = "https://github.com/trending";
        tokio::spawn(async move {
            if let Err(e) = browser::stream_screenshots(url, tx).await {
                tracing::error!("Failed to stream screenshots: {}", e);
            }
        });

        // Await screenshot
        if let Some(snapshot) = rx.recv().await {
            tracing::info!("Screenshot received: {}", snapshot.title);

            // Convert to GraphicData
            let graphic_data = rgb_image_to_graphic_data(&snapshot.image);
            tracing::info!(
                "GraphicData created: {}x{}, {} bytes, ID {:?}",
                graphic_data.width,
                graphic_data.height,
                graphic_data.pixels.len(),
                graphic_data.id
            );

            // Send UpdateGraphics event via EventProxy
            let queues = UpdateQueues {
                pending: vec![graphic_data],
                remove_queue: Vec::new(),
            };

            proxy_clone.send_event(
                RioEventType::Rio(RioEvent::UpdateGraphics {
                    route_id: 0,
                    queues,
                }),
                window_id,
            );

            tracing::info!("✓ Graphics injected via UpdateGraphics event!");
        } else {
            tracing::warn!("Screenshot channel closed without data");
        }
    });

    // Create rioterm Application
    let app = rioterm::Application::new(config, None, &event_loop);

    // Wrap with oneshot sender
    let mut app_wrapper = ApplicationWrapper::new(app, window_tx);

    tracing::info!("Running rioterm application...");

    // Block main thread on event loop (Runtime workers continue in background)
    event_loop.run_app(&mut app_wrapper)?;

    Ok(())
}
