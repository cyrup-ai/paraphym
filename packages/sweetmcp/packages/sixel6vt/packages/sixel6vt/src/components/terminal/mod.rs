use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rio_backend::{
    clipboard::Clipboard,
    // Assuming PtyConfig is public under config
    config::{Config as RioConfig}, // PtyConfig removed, Shell removed
    event::RioEvent, // Import only RioEvent
    performer::handler::Processor,
    sugarloaf::{
        font::FontLibrary,
        layout::RootStyle,
        SugarloafWindow,
        SugarloafWindowSize,
        Sugarloaf,
        SugarloafRenderer,
        // Use the error type that contains the instance
        SugarloafWithErrors,
    },
    // Use the public terminal/pty APIs from rio-backend
    // Remove unused alias: crosswords::grid::Dimensions,
};
// Import Crosswords and alias as Terminal from the correct path
use rio_backend::crosswords::Crosswords as Terminal;
// Import color types for terminal text color rendering
use rio_backend::config::colors::{AnsiColor, ColorRgb, NamedColor};
// Import Pty, WinsizeBuilder and ProcessReadWrite trait from teletypewriter
use teletypewriter::{WinsizeBuilder, ProcessReadWrite};

use rio_window::{
    event_loop::ActiveEventLoop,
    window::Window, // Remove WindowId import
};
use std::{cell::RefCell, rc::Rc, io::{Read, Write, BufReader}};
use std::sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use anyhow::Result;
use tracing::{error, info, debug};

// Make TerminalPane generic over the EventListener type U
pub struct TerminalPane<U: rio_backend::event::EventListener + Clone + Send + 'static> { // Add Clone bound
    pub window: Window,
    pub terminal: Terminal<U>, // Use the generic parameter U
    pub pty_tx: mpsc::Sender<Vec<u8>>, // Channel to send data to PTY
    pub event_proxy: U, // Use the generic parameter U for the proxy type
    pub clipboard: Rc<RefCell<Clipboard>>,
    pub sugarloaf: Sugarloaf<'static>,
    parser: Processor, // Parser for processing terminal escape sequences and sixel data
    
    // Configuration values needed for correct dimension calculations
    font_size: f32,
    line_height: f32,
    padding_x: f32,
    padding_y: [f32; 2],
    
    // Thread management resources
    running: Arc<AtomicBool>,
    reader_thread: Option<JoinHandle<()>>,
    writer_thread: Option<JoinHandle<()>>,
}

// Helper function to map NamedColor to RGBA values
fn map_named_color_to_rgba(color: NamedColor) -> [f32; 4] {
    match color {
        NamedColor::Black => [0.0, 0.0, 0.0, 1.0],
        NamedColor::Red => [0.804, 0.192, 0.192, 1.0],  // (205, 49, 49)
        NamedColor::Green => [0.051, 0.737, 0.475, 1.0],  // (13, 188, 121)
        NamedColor::Yellow => [0.898, 0.898, 0.063, 1.0],  // (229, 229, 16)
        NamedColor::Blue => [0.141, 0.447, 0.784, 1.0],  // (36, 114, 200)
        NamedColor::Magenta => [0.737, 0.247, 0.737, 1.0],  // (188, 63, 188)
        NamedColor::Cyan => [0.067, 0.659, 0.804, 1.0],  // (17, 168, 205)
        NamedColor::White => [0.898, 0.898, 0.898, 1.0],  // (229, 229, 229)
        NamedColor::LightBlack => [0.333, 0.333, 0.333, 1.0],
        NamedColor::LightRed => [1.0, 0.333, 0.333, 1.0],
        NamedColor::LightGreen => [0.333, 1.0, 0.333, 1.0],
        NamedColor::LightYellow => [1.0, 1.0, 0.333, 1.0],
        NamedColor::LightBlue => [0.333, 0.333, 1.0, 1.0],
        NamedColor::LightMagenta => [1.0, 0.333, 1.0, 1.0],
        NamedColor::LightCyan => [0.333, 1.0, 1.0, 1.0],
        NamedColor::LightWhite => [1.0, 1.0, 1.0, 1.0],
        NamedColor::Foreground => [0.898, 0.898, 0.898, 1.0],  // Default foreground
        NamedColor::Background => [0.078, 0.078, 0.078, 1.0],  // Default background
        _ => [1.0, 1.0, 1.0, 1.0],  // Fallback white
    }
}

// Helper function to map indexed color (0-255) to RGBA values
fn map_indexed_color_to_rgba(idx: u8) -> [f32; 4] {
    match idx {
        0..=15 => {
            // Standard ANSI colors - map to NamedColor
            let named = match idx {
                0 => NamedColor::Black,
                1 => NamedColor::Red,
                2 => NamedColor::Green,
                3 => NamedColor::Yellow,
                4 => NamedColor::Blue,
                5 => NamedColor::Magenta,
                6 => NamedColor::Cyan,
                7 => NamedColor::White,
                8 => NamedColor::LightBlack,
                9 => NamedColor::LightRed,
                10 => NamedColor::LightGreen,
                11 => NamedColor::LightYellow,
                12 => NamedColor::LightBlue,
                13 => NamedColor::LightMagenta,
                14 => NamedColor::LightCyan,
                15 => NamedColor::LightWhite,
                _ => NamedColor::White,  // Unreachable but safe
            };
            map_named_color_to_rgba(named)
        }
        16..=231 => {
            // 6x6x6 RGB cube
            let idx = idx - 16;
            let r = idx / 36;
            let g = (idx % 36) / 6;
            let b = idx % 6;
            
            // Map 0-5 to RGB values
            let to_rgb = |v: u8| -> f32 {
                match v {
                    0 => 0.0,
                    1 => 0.373,  // 95/255
                    2 => 0.498,  // 135/255
                    3 => 0.686,  // 175/255
                    4 => 0.843,  // 215/255
                    5 => 1.0,    // 255/255
                    _ => 0.0,
                }
            };
            
            [to_rgb(r), to_rgb(g), to_rgb(b), 1.0]
        }
        232..=255 => {
            // Grayscale ramp
            let gray = ((idx - 232) as f32 * 10.0 + 8.0) / 255.0;
            [gray, gray, gray, 1.0]
        }
    }
}

// Implement methods for the generic TerminalPane
impl<U: rio_backend::event::EventListener + Clone + Send + 'static> TerminalPane<U> { // Add Clone bound
    pub fn new(
        active_event_loop: &ActiveEventLoop, // Use ActiveEventLoop
        event_proxy: U, // Use the generic parameter U
        config: &RioConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> { // Specify Error type
        // Configure window
        let window_builder = Window::default_attributes()
            .with_title("Rio Terminal")
            .with_transparent(config.window.opacity < 1.0);

        // Create window using ActiveEventLoop
        let window = active_event_loop.create_window(window_builder)?;

        // Create font library
        let (font_library, _) = FontLibrary::new(config.fonts.clone());

        // Create sugarloaf window
        let sugarloaf_window = SugarloafWindow {
            // Use HasWindowHandle trait
            handle: window.window_handle()?.into(),
            display: window.display_handle()?.into(),
            scale: window.scale_factor() as f32,
            size: SugarloafWindowSize {
                width: window.inner_size().width as f32,
                height: window.inner_size().height as f32,
            },
        };

        // Create sugarloaf instance
        let sugarloaf = match Sugarloaf::new(
            sugarloaf_window,
            SugarloafRenderer::default(),
            &font_library,
            RootStyle::new(
                window.scale_factor() as f32,
                config.fonts.size,
                config.line_height,
            ),
        ) {
            Ok(instance) => instance,
            // Use the correct error type from Sugarloaf - destructure Box
            Err(boxed_err) => boxed_err.instance, // Extract instance from Box<SugarloafWithErrors>
        };

        // Get dimensions for terminal
        let _scale = window.scale_factor() as f32; // Unused but kept for clarity
        let width_u = window.inner_size().width;
        let height_u = window.inner_size().height;
        let width = width_u as f32;
        let height = height_u as f32;

        // Calculate terminal dimensions accounting for padding
        let padding_w = config.padding_x * 2.0; // left + right
        let padding_h = config.padding_y[0] + config.padding_y[1]; // top + bottom
        let avail_w = (width - padding_w).max(0.0);
        let avail_h = (height - padding_h).max(0.0);

        let cols = (avail_w / config.fonts.size).floor() as usize;
        let lines = (avail_h / (config.fonts.size * config.line_height)).floor() as usize;
        let cols = cols.max(1);
        let lines = lines.max(1);
        let terminal_size = (cols, lines);

        // Create CrosswordsSize for Terminal::new - This needs to implement Dimensions
        let cross_size = rio_backend::crosswords::CrosswordsSize::new_with_dimensions(cols, lines, width_u, height_u, 0, 0); // Assuming 0 for square width/height initially

        // Get cursor shape and blinking setting from config
        let cursor_shape = config.cursor.shape;
            
        let terminal = Terminal::new(
            // Correct argument order: Dimensions, CursorStyle, EventListener, WindowId, route_id
            cross_size, // Pass CrosswordsSize
            cursor_shape, // Pass cursor shape
            event_proxy.clone(), // Pass event proxy
            window.id(),
            0, // Default route_id
        ); // Remove ? operator, new doesn't return Result

        // Create PTY using public API
        // Use teletypewriter::WinsizeBuilder and construct manually
        let pty_spawn_builder = WinsizeBuilder { // Construct manually
            rows: terminal_size.1 as u16, // rows
            cols: terminal_size.0 as u16, // cols
            // Add missing width and height fields
            width: width_u as u16,
            height: height_u as u16,
        };
        // Set critical terminal environment variables before creating PTY
        // From frontends/rioterm/src/main.rs example
        // SAFETY: Setting environment variables before spawning subprocess
        unsafe {
            std::env::set_var("TERM", "xterm-256color");
            std::env::set_var("TERM_PROGRAM", "rio");
            std::env::set_var("TERM_PROGRAM_VERSION", "1.0.0"); // Simulate a version
            std::env::set_var("COLORTERM", "truecolor");
        }
        
        // Get shell path using same approach as the spawn.rs example
        use std::borrow::Cow;
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        let shell = Cow::Borrowed(shell_path.as_str());
        info!("Using shell: {}", shell);
        
        // Unix (Linux and macOS) – use the portable create_pty API
        #[cfg(unix)]
        let mut process = teletypewriter::create_pty_with_spawn(
            &shell, 
            vec![], 
            &config.working_dir,
            pty_spawn_builder.cols,
            pty_spawn_builder.rows,
        )?;

        // Windows – still use conpty for cmd.exe
        #[cfg(windows)]
        let mut process = teletypewriter::windows::new(
            "cmd.exe",
            &config.working_dir,
            pty_spawn_builder.cols,
            pty_spawn_builder.rows,
        )?;

        // Create channels for PTY communication - we'll need this for the application
        let (pty_tx, pty_rx) = mpsc::channel::<Vec<u8>>();
        
        // Log status
        info!("PTY created successfully with dimensions {}x{}", 
             pty_spawn_builder.cols, pty_spawn_builder.rows);
        
        // Using the same pattern as spawn.rs: direct command sequence
        // Send initialization commands immediately to the PTY
        // This is crucial for shell initialization
        info!("Sending initialization commands to PTY");
        
        // First, send individual characters as in the example
        process.writer().write_all(b"1").map_err(|e| {
            error!("Failed to write init chars to PTY: {}", e);
            anyhow::anyhow!("Failed to write to PTY: {}", e)
        })?;
        process.writer().write_all(b"2").map_err(|e| {
            error!("Failed to write init chars to PTY: {}", e);
            anyhow::anyhow!("Failed to write to PTY: {}", e)
        })?;
        
        // Then send actual commands with newlines
        let init_commands = [
            b"clear\n".as_slice(),
            b"stty -echo\n".as_slice(), // Disable echo for cleaner display
            b"stty cols 50 rows 50\n".as_slice(), // Set terminal size explicitly
            b"echo 'Terminal initialized successfully'\n".as_slice(),
            b"pwd\n".as_slice(),          // Show current directory
            b"ls -la\n".as_slice(),       // List files
            b"export PS1='$ '\n".as_slice(), // Set a simple prompt
        ];
        
        for cmd in &init_commands {
            process.writer().write_all(cmd).map_err(|e| {
                error!("Failed to write command to PTY: {}", e);
                anyhow::anyhow!("Failed to write to PTY: {}", e)
            })?;
            process.writer().flush().map_err(|e| {
                error!("Failed to flush PTY writer: {}", e);
                anyhow::anyhow!("Failed to flush PTY: {}", e)
            })?;
            
            // Add a small delay between commands to ensure they're processed properly
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        
        // Create shared running flag
        let running = Arc::new(AtomicBool::new(true));
        
        // Clone event proxy for threads and create window id
        let event_proxy_clone = event_proxy.clone();
        let window_id = window.id();
        
        // Create cloned reader for reader thread
        #[cfg(not(windows))]
        let reader = process.reader().try_clone().map_err(|e| {
            error!("Failed to clone PTY reader: {}", e);
            anyhow::anyhow!("Failed to clone PTY reader: {}", e)
        })?;
        
        #[cfg(windows)]
        let reader = process.reader().try_clone().map_err(|e| {
            error!("Failed to clone PTY reader: {}", e);
            anyhow::anyhow!("Failed to clone PTY reader: {}", e)
        })?;
        
        // Clone running flag for reader thread
        let reader_running = running.clone();
        
        // Set up reader thread with proper resource management
        let reader_thread = thread::spawn(move || {
            info!("PTY reader thread started");
            
            // Buffer for batching bytes before sending events
            let mut buffer = Vec::with_capacity(8192);
            let mut last_send = std::time::Instant::now();
            
            // Use BufReader for efficient reading
            let mut buf_reader = BufReader::new(reader);
            let mut read_buffer = [0u8; 4096]; // Read up to 4KB at a time
            
            loop {
                // Check if we should continue running
                if !reader_running.load(Ordering::SeqCst) {
                    debug!("Reader thread received shutdown signal");
                    break;
                }
                
                match buf_reader.read(&mut read_buffer) {
                    Ok(0) => {
                        // EOF reached
                        debug!("PTY reader reached EOF");
                        break;
                    }
                    Ok(n) => {
                        // Add bytes to buffer
                        buffer.extend_from_slice(&read_buffer[..n]);
                        
                        // Send if buffer is getting full (>4KB) or it's been >10ms since last send
                        let should_send = buffer.len() >= 4096 || last_send.elapsed().as_millis() > 10;
                        
                        if should_send && !buffer.is_empty() {
                            let text = String::from_utf8_lossy(&buffer).to_string();
                            let event = rio_backend::event::RioEvent::PtyWrite(text);
                            event_proxy_clone.send_event(event, window_id);
                            buffer.clear();
                            last_send = std::time::Instant::now();
                        }
                    }
                    Err(e) => {
                        // Check if this is just a "would block" error
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            // Flush any buffered data before waiting
                            if !buffer.is_empty() {
                                let text = String::from_utf8_lossy(&buffer).to_string();
                                let event = rio_backend::event::RioEvent::PtyWrite(text);
                                event_proxy_clone.send_event(event, window_id);
                                buffer.clear();
                                last_send = std::time::Instant::now();
                            }
                            // Wait a bit before trying again
                            std::thread::sleep(std::time::Duration::from_millis(1));
                            continue;
                        }
                        
                        // Only log as error if thread is still supposed to be running
                        if reader_running.load(Ordering::SeqCst) {
                            error!("Failed to read from PTY: {}", e);
                        } else {
                            debug!("PTY reader closed during shutdown: {}", e);
                        }
                        break;
                    }
                }
            }
            
            // Flush any remaining buffered data
            if !buffer.is_empty() {
                let text = String::from_utf8_lossy(&buffer).to_string();
                let event = rio_backend::event::RioEvent::PtyWrite(text);
                event_proxy_clone.send_event(event, window_id);
            }
            
            info!("PTY reader thread terminated");
        });
        
        // Clone running flag for writer thread
        let writer_running = running.clone();
        
        // Set up writer thread with proper resource management
        let writer_thread = thread::spawn(move || {
            info!("PTY writer thread started");
            
            // Handle incoming commands from the application
            while writer_running.load(Ordering::SeqCst) {
                // Use timeout recv to allow checking the running flag periodically
                match pty_rx.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(data) => {
                        debug!("Received command to send to PTY ({} bytes)", data.len());
                        
                        // Only log the first 20 bytes to avoid flooding logs with large payloads
                        if data.len() > 20 {
                            let preview = String::from_utf8_lossy(&data[0..20]);
                            debug!("Command preview: {:?}...", preview);
                        } else {
                            debug!("Command: {:?}", String::from_utf8_lossy(&data));
                        }
                        
                        if let Err(e) = process.writer().write_all(&data) {
                            // Check if this is just a "would block" error
                            if e.kind() == std::io::ErrorKind::WouldBlock {
                                // For writes, we should retry after a short delay
                                std::thread::sleep(std::time::Duration::from_millis(10));
                                // Try again
                                if let Err(e2) = process.writer().write_all(&data) {
                                    if writer_running.load(Ordering::SeqCst) && e2.kind() != std::io::ErrorKind::WouldBlock {
                                        error!("Failed to write to PTY after retry: {}", e2);
                                    }
                                }
                            } else if writer_running.load(Ordering::SeqCst) {
                                error!("Failed to write to PTY: {}", e);
                                break;
                            } else {
                                debug!("PTY write error during shutdown: {}", e);
                                break;
                            }
                        }
                        
                        if let Err(e) = process.writer().flush() {
                            if writer_running.load(Ordering::SeqCst) {
                                error!("Failed to flush PTY writer: {}", e);
                            } else {
                                debug!("PTY flush error during shutdown: {}", e);
                            }
                            break;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Timeout is normal, just check the running flag
                        continue;
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        debug!("PTY writer channel disconnected");
                        break;
                    }
                }
            }
            info!("PTY writer thread terminated");
        });
        
// The reader thread is already created above

        // Get clipboard using public API
        let clipboard = unsafe {
            // Use HasDisplayHandle trait
            Clipboard::new(window.display_handle()?.as_raw())
        };

        // No need to manually write initial content, Pty handles shell startup

        Ok(Self {
            window,
            terminal, // Assign created terminal
            pty_tx, // Assign the pty writer channel
            event_proxy,
            clipboard: Rc::new(RefCell::new(clipboard)),
            sugarloaf,
            parser: Processor::new(), // Initialize the parser for terminal byte processing
            // Store config values for resize calculations
            font_size: config.fonts.size,
            line_height: config.line_height,
            padding_x: config.padding_x,
            padding_y: config.padding_y,
            // Store thread management resources
            running,
            reader_thread: Some(reader_thread),
            writer_thread: Some(writer_thread),
        })
    }

    // Render terminal content using Sugarloaf
    pub fn render(&mut self) {
        use rio_backend::crosswords::pos::{Column, Line};
        use rio_backend::crosswords::square::Flags;
        use rio_backend::sugarloaf::{FragmentStyle, Graphic};
        
        // Note: Graphics are uploaded via UpdateGraphics event handler in app.rs
        // The event is sent by Crosswords.send_graphics_updates() after sixel parsing
        
        // Get terminal dimensions
        let rows = self.terminal.screen_lines();
        let cols = self.terminal.columns();
        
        // DEBUG: Check if there's any content in the grid
        let mut total_chars = 0;
        let mut first_10_chars = String::new();
        for row_idx in 0..rows.min(3) {
            let row = &self.terminal.grid[Line(row_idx as i32)];
            for col_idx in 0..cols.min(20) {
                let c = row[Column(col_idx)].c;
                if c != ' ' && c != '\0' {
                    total_chars += 1;
                    if first_10_chars.len() < 50 {
                        first_10_chars.push(c);
                    }
                }
            }
        }
        info!("RENDER: {}x{} grid, {} non-space chars, first chars: {:?}", 
              rows, cols, total_chars, first_10_chars);
        
        // Get the content builder from Sugarloaf
        let content = self.sugarloaf.content();
        
        // Create or get the rich text ID for terminal content
        // Use a fixed ID for the main terminal display
        let rich_text_id = 0;
        content.sel(rich_text_id);
        content.clear();
        
        // Iterate through visible rows
        for row_idx in 0..rows {
            let mut line_content = String::with_capacity(cols);
            let mut last_style = FragmentStyle::default();
            let mut has_content = false;
            
            // Get the row from the terminal grid
            let row = &self.terminal.grid[Line(row_idx as i32)];
            
            for col_idx in 0..cols {
                let square = &row[Column(col_idx)];
                
                // Skip wide char spacers
                if square.flags.contains(Flags::WIDE_CHAR_SPACER) {
                    continue;
                }
                
                let mut style = FragmentStyle::default();
                
                // Handle graphics (sixel)
                if square.flags.contains(Flags::GRAPHICS) {
                    // Flush any pending text
                    if !line_content.is_empty() {
                        content.add_text_on_line(row_idx, &line_content, last_style);
                        line_content.clear();
                    }
                    
                    // Add graphic
                    if let Some(graphics) = square.graphics() {
                        if let Some(graphic) = graphics.first() {
                            style.media = Some(Graphic {
                                id: graphic.texture.id,
                                offset_x: graphic.offset_x,
                                offset_y: graphic.offset_y,
                            });
                            // Add a space with the graphic style
                            content.add_text_on_line(row_idx, " ", style);
                            has_content = true;
                        }
                    }
                } else {
                    // Regular text character
                    let c = if square.c == '\t' { ' ' } else { square.c };
                    
                    // Get foreground color from cell attributes
                    style.color = match &square.fg {
                        AnsiColor::Named(named) => {
                            // Use named color mapping
                            map_named_color_to_rgba(*named)
                        }
                        AnsiColor::Spec(rgb) => {
                            // Direct RGB specification - convert to RGBA
                            [
                                rgb.r as f32 / 255.0,
                                rgb.g as f32 / 255.0,
                                rgb.b as f32 / 255.0,
                                1.0,
                            ]
                        }
                        AnsiColor::Indexed(idx) => {
                            // Indexed color - look up in palette
                            map_indexed_color_to_rgba(*idx)
                        }
                    };
                    
                    // If style changed, flush previous content
                    if has_content && style != last_style && !line_content.is_empty() {
                        content.add_text_on_line(row_idx, &line_content, last_style);
                        line_content.clear();
                    }
                    
                    line_content.push(c);
                    last_style = style;
                    has_content = true;
                }
            }
            
            // Flush any remaining content for this line
            if !line_content.is_empty() {
                content.add_text_on_line(row_idx, &line_content, last_style);
            }
        }
        
        // Build the content
        content.build();
        
        // Render
        self.sugarloaf.render();
        
        tracing::debug!("Terminal rendered");
    }

    // Handle window resize
    pub fn resize(&mut self, width: u32, height: u32) {
        let dpr = self.window.scale_factor();
        let _scale = dpr as f32; // Keep but unused
        self.sugarloaf.resize(width, height);

        // Calculate terminal dimensions accounting for padding
        let padding_w = self.padding_x * 2.0;
        let padding_h = self.padding_y[0] + self.padding_y[1];
        let avail_w = (width as f32 - padding_w).max(0.0);
        let avail_h = (height as f32 - padding_h).max(0.0);

        let cols = (avail_w / self.font_size).floor() as usize;
        let lines = (avail_h / (self.font_size * self.line_height)).floor() as usize;

        let cols = cols.max(1);
        let lines = lines.max(1);
        
        let cross_size = rio_backend::crosswords::CrosswordsSize::new_with_dimensions(
            cols,
            lines,
            width,
            height,
            0,
            0
        );
        self.terminal.resize(cross_size);

        // Create the resize command with window dimensions (guard casts)
        let ws_cols = cols.min(u16::MAX as usize) as u16;
        let ws_rows = lines.min(u16::MAX as usize) as u16;
        let ws_w = width.min(u16::MAX as u32) as u16;
        let ws_h = height.min(u16::MAX as u32) as u16;
        let resize_info = WinsizeBuilder { 
            cols: ws_cols, 
            rows: ws_rows, 
            width: ws_w, 
            height: ws_h 
        };
        // Serialize the resize info and send to PTY
        let resize_sequence = format!(
            "\x1b[8;{};{}t", 
            resize_info.rows,
            resize_info.cols
        ).into_bytes();
        
        if let Err(e) = self.pty_tx.send(resize_sequence) {
            error!("Failed to send resize command to PTY: {}", e);
        } else {
            info!("Terminal resized to {}x{}", resize_info.cols, resize_info.rows);
        }
    }
    
    /// Handle output from the PTY process or injected content (like sixel graphics)
    /// 
    /// This processes data that should be written to the terminal display.
    /// Uses the batched parser to handle escape sequences, sixel graphics, and text.
    pub fn handle_pty_output(&mut self, data: &[u8]) -> anyhow::Result<()> {
        info!("handle_pty_output called with {} bytes", data.len());
        
        // Skip processing if terminal is shutting down
        if !self.running.load(Ordering::SeqCst) {
            error!("Skipping PTY output processing - terminal is shutting down (GOT {} BYTES!)", data.len());
            return Ok(());
        }

        // Debug: Log when we receive data
        if data.len() > 100 {
            info!("Processing {} bytes from PTY (large data - likely sixel)", data.len());
            // Log first 50 bytes to see if it's a sixel sequence
            let preview = &data[..data.len().min(50)];
            info!("First bytes: {:?}", String::from_utf8_lossy(preview));
        }

        // Process the bytes through the parser which will update the terminal grid
        // This handles all escape sequences, sixel graphics, and text rendering
        info!("Calling parser.advance with {} bytes", data.len());
        
        // Catch panics in the parser
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.parser.advance(&mut self.terminal, data);
        }));
        
        match result {
            Ok(_) => {
                info!("parser.advance completed successfully");
            }
            Err(e) => {
                error!("PARSER PANICKED: {:?}", e);
            }
        }
        
        // Check if graphics were created
        let graphics_count = self.terminal.graphics.pending.len();
        if graphics_count > 0 {
            info!("Parser created {} graphics!", graphics_count);
        }
        
        // Request redraw to display the updates
        self.window.request_redraw();
        
        Ok(())
    }
    
    /// Clean up resources before dropping the terminal
    pub fn cleanup(&mut self) {
        // Set running flag to false to signal threads to shut down
        self.running.store(false, Ordering::SeqCst);
        info!("Signaled terminal threads to shut down");
        
        // Join reader thread if it exists
        if let Some(reader_thread) = self.reader_thread.take() {
            // Give thread a bit of time to exit cleanly
            if let Err(e) = reader_thread.join() {
                error!("Failed to join reader thread: {:?}", e);
            } else {
                debug!("Reader thread joined successfully");
            }
        }
        
        // Join writer thread if it exists
        if let Some(writer_thread) = self.writer_thread.take() {
            // Give thread a bit of time to exit cleanly
            if let Err(e) = writer_thread.join() {
                error!("Failed to join writer thread: {:?}", e);
            } else {
                debug!("Writer thread joined successfully");
            }
        }
        
        info!("Terminal cleanup completed");
    }
}

// Implement Drop for TerminalPane to ensure cleanup happens
impl<U: rio_backend::event::EventListener + Clone + Send + 'static> Drop for TerminalPane<U> {
    fn drop(&mut self) {
        self.cleanup();
    }
}
