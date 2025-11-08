//! Console output writer

use super::TextWriter;
use crate::error::Result;
use std::io::{self, Write};

/// Writer that outputs to stdout
pub struct ConsoleWriter;

impl ConsoleWriter {
    /// Create a new console writer
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl TextWriter for ConsoleWriter {
    fn write(&mut self, text: &str) -> Result<()> {
        print!("{}", text);
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        io::stdout().flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_writer_creation() {
        let _ = ConsoleWriter::new();
        let _ = ConsoleWriter::default();
    }

    #[test]
    fn test_console_writer_write() {
        let mut writer = ConsoleWriter::new();
        // Note: actual output goes to stdout, which we can't easily test
        // This just ensures no panics
        assert!(writer.write("test").is_ok());
        assert!(writer.flush().is_ok());
    }
}
