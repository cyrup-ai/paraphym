// sixel6vt - Terminal with browser screenshots as sixel graphics
// Uses rioterm's Application directly

mod browser;
mod renderer;

use anyhow::Result;
use rio_backend::config::Config;
use rio_backend::event::{EventPayload, EventProxy, RioEvent, RioEventType};
use rio_window::event_loop::EventLoop;
use rio_window::window::WindowId;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tracing_subscriber;

fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("sixel6vt=debug,rioterm=info,rio_backend=info")
        .init();

    tracing::info!("Starting sixel6vt using rioterm");

    // Create tokio runtime for browser operations
    let runtime = Arc::new(Runtime::new()?);

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
    let mut app = rioterm::Application::new(config, None, &event_loop);

    // Spawn background task to capture and inject browser screenshots
    let runtime_clone = runtime.clone();
    let proxy_clone = event_proxy.clone();
    std::thread::spawn(move || {
        runtime_clone.block_on(async {
            // Wait for terminal to initialize and window to be created
            tokio::time::sleep(Duration::from_secs(5)).await;

            tracing::info!("Starting browser screenshot capture...");

            // Create channel for screenshots
            let (tx, mut rx) = mpsc::channel(1);

            // Capture screenshot
            let url = "https://github.com/trending";
            tokio::spawn(async move {
                if let Err(e) = browser::stream_screenshots(url, tx).await {
                    tracing::error!("Failed to stream screenshots: {}", e);
                }
            });

            // Wait for screenshot
            if let Some(snapshot) = rx.recv().await {
                tracing::info!("Screenshot received, encoding to sixel...");

                // Encode to sixel
                let sixel = renderer::encode_sixel(&snapshot.image);
                tracing::info!("Sixel encoded ({} bytes), sending to event loop...", sixel.len());

                // Send custom event to inject sixel
                // We'll use a dummy WindowId since rioterm will iterate all windows
                let sixel_event = EventPayload::new(
                    RioEventType::Rio(RioEvent::PtyWrite(format!("\r\n{}\r\n", sixel))),
                    unsafe { WindowId::dummy() }
                );
                
                proxy_clone.send_event(RioEventType::Rio(RioEvent::PtyWrite(sixel)), unsafe { WindowId::dummy() });
                tracing::info!("Sixel event sent!");
            }
        });
    });

    // Run rioterm's application
    tracing::info!("Running rioterm application...");
    app.run(event_loop).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    Ok(())
}
