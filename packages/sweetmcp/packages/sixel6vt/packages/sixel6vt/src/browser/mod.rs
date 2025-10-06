use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;
use std::sync::Arc;
use image::RgbImage;
use tokio::sync::mpsc::Sender;

/// A snapshot of browser content including both the rendered image and page title
pub struct BrowserSnapshot {
    pub image: RgbImage,
    pub title: String,
}

/// Streams screenshots as BrowserSnapshot to the provided channel, given a URL.
pub async fn stream_screenshots(url: &str, tx: Sender<BrowserSnapshot>) -> Result<()> {
    // Create a temporary directory for Chrome
    let temp_dir = Arc::new(tempfile::Builder::new()
        .prefix("rio-ext-browser")
        .tempdir()?);
    let config = BrowserConfig::builder()
        .user_data_dir(temp_dir.path())
        .args(vec!["--no-sandbox", "--disable-dev-shm-usage"])
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?;
    let (mut browser, mut handler) = Browser::launch(config).await?;
    // Spawn handler for browser events
    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            let _ = h;
        }
    });
    // Create a new page and navigate
    let page = browser.new_page(url).await?;
    
    // Fetch the page title with fallback to URL
    let title = page.get_title().await
        .ok()
        .flatten()
        .unwrap_or_else(|| url.to_string());
    
    // Take a screenshot (could be looped for periodic shots)
    let screenshot_data = page.screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(true)
            .build(),
    ).await?;
    // Decode PNG to RgbImage
    let img = image::load_from_memory(&screenshot_data)?.to_rgb8();
    
    // Create and send snapshot with both image and title
    let snapshot = BrowserSnapshot {
        image: img,
        title,
    };
    let _ = tx.send(snapshot).await;
    
    // Properly close the browser to avoid background kill warning
    browser.close().await?;
    
    Ok(())
}

// (Legacy WebBrowser struct and impl removed for modular interface)
