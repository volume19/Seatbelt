//! Output sink trait definition

use crate::commands::CommandDTO;
use crate::error::Result;

/// Output sink for command results
///
/// This trait defines the interface for all output destinations.
/// Implementations include text output, JSON file output, etc.
pub trait OutputSink: Send {
    /// Write a command output DTO
    fn write_output(&mut self, dto: &dyn CommandDTO) -> Result<()>;

    /// Write a host/console message
    fn write_host(&mut self, msg: &str);

    /// Write an error message
    fn write_error(&mut self, msg: &str);

    /// Write a verbose/debug message
    fn write_verbose(&mut self, msg: &str);

    /// Write a warning message
    fn write_warning(&mut self, msg: &str);

    /// Get the accumulated output as a string (for string-based sinks)
    fn get_output(&self) -> String {
        String::new()
    }
}
