//! Text output sink

use super::{OutputSink, TextWriter};
use crate::commands::CommandDTO;
use crate::error::Result;

/// Output sink that writes text to a TextWriter
pub struct TextOutputSink<W: TextWriter> {
    writer: W,
    filter_results: bool,
}

impl<W: TextWriter> TextOutputSink<W> {
    /// Create a new text output sink
    pub fn new(writer: W, filter_results: bool) -> Self {
        Self {
            writer,
            filter_results,
        }
    }
}

impl<W: TextWriter> OutputSink for TextOutputSink<W> {
    fn write_output(&mut self, dto: &dyn CommandDTO) -> Result<()> {
        let text = dto.format_text();
        self.writer.writeln(&text)?;
        self.writer.flush()?;
        Ok(())
    }

    fn write_host(&mut self, msg: &str) {
        let _ = self.writer.writeln(msg);
    }

    fn write_error(&mut self, msg: &str) {
        let _ = self.writer.writeln(&format!("  [!] {}", msg));
    }

    fn write_verbose(&mut self, msg: &str) {
        if !self.filter_results {
            let _ = self.writer.writeln(&format!("  [*] {}", msg));
        }
    }

    fn write_warning(&mut self, msg: &str) {
        let _ = self.writer.writeln(&format!("  [W] {}", msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::dto::HostDTO;

    struct MockWriter {
        buffer: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                buffer: String::new(),
            }
        }
    }

    impl TextWriter for MockWriter {
        fn write(&mut self, text: &str) -> Result<()> {
            self.buffer.push_str(text);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_text_sink() {
        let writer = MockWriter::new();
        let mut sink = TextOutputSink::new(writer, false);

        sink.write_host("Test message");
        sink.write_error("Error message");
        sink.write_warning("Warning message");

        let dto = HostDTO::new("DTO message");
        sink.write_output(&dto).unwrap();
    }
}
