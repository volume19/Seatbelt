//! Text writer trait

use crate::error::Result;

/// Text writer abstraction for output destinations
pub trait TextWriter: Send {
    /// Write text to the output
    fn write(&mut self, text: &str) -> Result<()>;

    /// Flush any buffered output
    fn flush(&mut self) -> Result<()>;

    /// Write a line (text + newline)
    fn writeln(&mut self, text: &str) -> Result<()> {
        self.write(text)?;
        self.write("\n")?;
        Ok(())
    }
}
