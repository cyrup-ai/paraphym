use log;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Console logger service supporting styled output
#[derive(Clone)]
pub struct ConsoleLogger {
    color_choice: ColorChoice,
}

impl ConsoleLogger {
    /// Create a new console logger with automatic color detection
    pub fn new() -> Self {
        Self {
            color_choice: ColorChoice::Auto,
        }
    }
    
    /// Create a console logger with specific color choice
    pub fn with_color_choice(color_choice: ColorChoice) -> Self {
        Self { color_choice }
    }

    /// Log a warning message (yellow)
    pub fn warn(&self, message: &str) {
        let mut stdout = StandardStream::stdout(self.color_choice);
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = writeln!(&mut stdout, "⚠ {}", message);
        let _ = stdout.reset();
        log::warn!("{}", message);
    }

    /// Log a success message (green)
    pub fn success(&self, message: &str) {
        let mut stdout = StandardStream::stdout(self.color_choice);
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        let _ = writeln!(&mut stdout, "✓ {}", message);
        let _ = stdout.reset();
        log::info!("{}", message);
    }

    /// Log an error message (red)
    pub fn error(&self, message: &str) {
        let mut stderr = StandardStream::stderr(self.color_choice);
        let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
        let _ = writeln!(&mut stderr, "✗ {}", message);
        let _ = stderr.reset();
        log::error!("{}", message);
    }
}
