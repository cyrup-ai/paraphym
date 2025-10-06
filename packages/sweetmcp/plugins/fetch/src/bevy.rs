use async_trait::async_trait;
use base64::Engine;
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
    },
};
use image::DynamicImage;
use log::{error, warn};
use scraper::{Html, Selector, ElementRef};
use std::error::Error as StdError;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::chromiumoxide::{ContentFetcher, FetchResult};

#[derive(Debug)]
pub enum BevyRenderError {
    Setup(String),
    Render(String),
    Screenshot(String),
    Content(String),
}

impl fmt::Display for BevyRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BevyRenderError::Setup(e) => write!(f, "Setup error: {}", e),
            BevyRenderError::Render(e) => write!(f, "Render error: {}", e),
            BevyRenderError::Screenshot(e) => write!(f, "Screenshot error: {}", e),
            BevyRenderError::Content(e) => write!(f, "Content error: {}", e),
        }
    }
}

impl StdError for BevyRenderError {}

// Resource to store the HTML content
#[derive(Resource)]
struct HtmlContent(String);

// Resource to store the screenshot
#[derive(Resource)]
struct Screenshot(Option<DynamicImage>);

// Flag to indicate when rendering is complete
#[derive(Resource)]
struct RenderComplete(bool);

// Resource to track rendering state for completion detection
#[derive(Resource, Default)]
struct RenderState {
    expected_nodes: usize,
    spawned_nodes: usize,
    frames_rendered: u32,
    has_rendered: bool,
}

// Resource to store the render target handle
#[derive(Resource)]
struct RenderTarget(Handle<Image>);

// Mutable state to be shared between Bevy and the main thread
struct SharedState {
    content: Option<String>,
    screenshot_base64: Option<String>,
}

pub struct BevyRenderer;

impl BevyRenderer {
    fn setup_bevy_app(html_content: String) -> Result<String, BevyRenderError> {
        let shared_state = Arc::new(Mutex::new(SharedState {
            content: None,
            screenshot_base64: None,
        }));
        
        let shared_state_clone = shared_state.clone();

        // Create and run the Bevy app
        let mut app = App::new();
        
        app.insert_resource(HtmlContent(html_content))
            .insert_resource(Screenshot(None))
            .insert_resource(RenderComplete(false))
            .insert_resource(RenderState::default())
            .add_plugins(DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Web Content Renderer".to_string(),
                        resolution: (1280., 800.).into(),
                        visible: false, // Headless rendering
                        ..default()
                    }),
                    ..default()
                }
            ))
            .add_systems(Startup, setup_renderer_system)
            .add_systems(Update, (
                render_system,
                screenshot_system,
                increment_frame_counter,
                check_completion_system,
            ).chain());

        // Run the app with a custom runner that will terminate when rendering is complete
        app.add_systems(Update, move |render_complete: Res<RenderComplete>, 
                                    screenshot: Res<Screenshot>,
                                    html_content: Res<HtmlContent>| {
            if render_complete.0 {
                if let Some(image) = &screenshot.0 {
                    // Convert image to base64
                    let mut buffer = std::io::Cursor::new(Vec::new());
                    if let Err(e) = image.write_to(&mut buffer, image::ImageOutputFormat::Png) {
                        error!("Failed to encode screenshot: {}", e);
                    } else {
                        let base64_image = base64::engine::general_purpose::STANDARD.encode(buffer.into_inner());
                        match shared_state_clone.lock() {
                            Ok(mut state) => {
                                state.content = Some(html_content.0.clone());
                                state.screenshot_base64 = Some(base64_image);
                            }
                            Err(e) => {
                                error!("Failed to lock shared state: {}", e);
                            }
                        }
                }
                std::process::exit(0);
            }
        });

        app.run();

        // Extract the results from the shared state
        let state = match shared_state.lock() {
            Ok(state) => state,
            Err(e) => return Err(format!("Failed to lock shared state: {}", e)),
        };
        
        if let Some(screenshot_base64) = &state.screenshot_base64 {
            Ok(screenshot_base64.clone())
        } else {
            Err(BevyRenderError::Screenshot("Failed to capture screenshot".to_string()))
        }
    }
}

// System to set up the renderer
fn setup_renderer_system(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create render target image
    let size = Extent3d {
        width: 1280,
        height: 800,
        depth_or_array_layers: 1,
    };
    
    let mut render_target = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("render_target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT 
                | TextureUsages::COPY_SRC 
                | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        asset_usage: RenderAssetUsages::all(),
        ..default()
    };
    
    render_target.resize(size);
    
    let render_target_handle = images.add(render_target);
    
    // Spawn camera rendering to texture using Required Components
    commands.spawn((
        Camera2d,
        Camera {
            target: bevy::render::camera::RenderTarget::Image(render_target_handle.clone()),
            ..default()
        },
    ));
    
    commands.insert_resource(RenderTarget(render_target_handle));
}

// System to render the HTML content
fn render_system(
    html_content: Res<HtmlContent>,
    mut commands: Commands,
    mut render_state: ResMut<RenderState>,
) {
    // Guard: only run once
    if render_state.has_rendered {
        return;
    }
    
    // Parse HTML with scraper
    let document = Html::parse_document(&html_content.0);
    
    // Create root container using Required Components
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::WHITE),
    )).id();
    
    // Count expected nodes for completion tracking
    if let Ok(body_selector) = Selector::parse("body") {
        if let Some(body) = document.select(&body_selector).next() {
            let node_count = count_html_nodes(body);
            render_state.expected_nodes = node_count;
            
            // Recursively spawn HTML elements
            spawn_html_element(body, root, &mut commands, &mut render_state);
        }
    }
    
    // Mark as rendered to prevent re-execution
    render_state.has_rendered = true;
}

/// Recursively spawn Bevy entities from HTML elements
fn spawn_html_element(
    element: ElementRef,
    parent: Entity,
    commands: &mut Commands,
    render_state: &mut RenderState,
) {
    let tag_name = element.value().name();
    
    // Map HTML elements to Bevy UI components
    match tag_name {
        "div" | "section" | "article" | "main" => {
            // Block-level container
            let entity = commands.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
            )).id();
            
            commands.entity(parent).add_child(entity);
            render_state.spawned_nodes += 1;
            
            // Recurse for children
            for child in element.children() {
                if let Some(child_element) = ElementRef::wrap(child) {
                    spawn_html_element(child_element, entity, commands, render_state);
                }
            }
        }
        "p" | "span" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            // Text elements
            let text_content = element.text().collect::<String>();
            if !text_content.trim().is_empty() {
                let font_size = match tag_name {
                    "h1" => 32.0,
                    "h2" => 28.0,
                    "h3" => 24.0,
                    "h4" => 20.0,
                    "h5" => 18.0,
                    "h6" => 16.0,
                    _ => 14.0,
                };
                
                let entity = commands.spawn((
                    Text::new(text_content.trim()),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Node {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                )).id();
                
                commands.entity(parent).add_child(entity);
                render_state.spawned_nodes += 1;
            }
        }
        "ul" | "ol" => {
            // List container
            let entity = commands.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
            )).id();
            
            commands.entity(parent).add_child(entity);
            render_state.spawned_nodes += 1;
            
            for child in element.children() {
                if let Some(child_element) = ElementRef::wrap(child) {
                    spawn_html_element(child_element, entity, commands, render_state);
                }
            }
        }
        "li" => {
            // List item
            let text_content = element.text().collect::<String>();
            if !text_content.trim().is_empty() {
                let entity = commands.spawn((
                    Text::new(format!("• {}", text_content.trim())),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Node {
                        margin: UiRect::vertical(Val::Px(2.0)),
                        ..default()
                    },
                )).id();
                
                commands.entity(parent).add_child(entity);
                render_state.spawned_nodes += 1;
            }
        }
        _ => {
            // Other elements: recurse through children
            for child in element.children() {
                if let Some(child_element) = ElementRef::wrap(child) {
                    spawn_html_element(child_element, parent, commands, render_state);
                }
            }
        }
    }
}

/// Count total HTML nodes for completion tracking
/// Only counts nodes that will actually be spawned
fn count_html_nodes(element: ElementRef) -> usize {
    let mut count = 0; // Don't count the body element itself
    for child in element.children() {
        if let Some(child_element) = ElementRef::wrap(child) {
            count += count_html_nodes_with_self(child_element);
        }
    }
    count
}

/// Count a single element and its children (helper)
fn count_html_nodes_with_self(element: ElementRef) -> usize {
    let tag_name = element.value().name();
    
    // Only count elements that spawn_html_element will actually spawn
    // This MUST match the spawning logic exactly!
    let self_count = match tag_name {
        "div" | "section" | "article" | "main" | "ul" | "ol" => 1,
        "p" | "span" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "li" => {
            if !element.text().collect::<String>().trim().is_empty() { 
                1  // Only count if has non-empty text
            } else { 
                0  // Empty text elements are not spawned
            }
        }
        _ => 0,  // All other elements (including body) are not spawned
    };
    
    let mut count = self_count;
    for child in element.children() {
        if let Some(child_element) = ElementRef::wrap(child) {
            count += count_html_nodes_with_self(child_element);
        }
    }
    count
}

// System to take a screenshot
fn screenshot_system(
    images: Res<Assets<Image>>,
    render_target: Res<RenderTarget>,
    mut screenshot: ResMut<Screenshot>,
    render_complete: Res<RenderComplete>,
) {
    if !render_complete.0 || screenshot.0.is_some() {
        return;
    }

    // Get the rendered image from the render target
    if let Some(image) = images.get(&render_target.0) {
        // Convert Bevy Image to DynamicImage
        match image.clone().try_into_dynamic() {
            Ok(dynamic_image) => {
                screenshot.0 = Some(dynamic_image);
            }
            Err(_) => {
                error!("Failed to convert render target to DynamicImage");
            }
        }
    }
}

// System to increment frame counter for completion tracking
fn increment_frame_counter(mut render_state: ResMut<RenderState>) {
    if render_state.spawned_nodes >= render_state.expected_nodes {
        render_state.frames_rendered += 1;
    }
}

// System to check when rendering is complete
fn check_completion_system(
    render_state: Res<RenderState>,
    mut render_complete: ResMut<RenderComplete>,
) {
    // Check if rendering is actually complete
    let all_nodes_spawned = render_state.spawned_nodes >= render_state.expected_nodes;
    let sufficient_frames = render_state.frames_rendered >= 2; // Allow layout to settle
    
    // Only mark complete when all criteria met
    if all_nodes_spawned && sufficient_frames && !render_complete.0 {
        render_complete.0 = true;
    }
}

#[async_trait]
impl ContentFetcher for BevyRenderer {
    async fn fetch_content(&self, url: &str) -> Result<FetchResult, Box<dyn StdError + Send + Sync>> {
        // First, fetch the content using Hyper
        let html_content = crate::hyper::HyperFetcher::fetch(url)
            .await
            .map_err(|e| BevyRenderError::Content(format!("Failed to fetch content: {}", e)))?;
        
        // Clean the HTML (remove scripts and styles)
        let cleaned_html = crate::hyper::HyperFetcher::clean_html(&html_content);
        let cleaned_html_clone = cleaned_html.clone();
        
        // Render the content and get a screenshot
        let screenshot_base64 = tokio::task::spawn_blocking(move || {
            BevyRenderer::setup_bevy_app(cleaned_html_clone)
        })
        .await
        .map_err(|e| BevyRenderError::Setup(format!("Failed to spawn rendering task: {}", e)))?
        .map_err(|e| BevyRenderError::Render(format!("Failed to render content: {}", e)))?;
        
        Ok(FetchResult {
            content: cleaned_html,
            screenshot_base64,
            content_type: "text/html".to_string(),
        })
    }
}
