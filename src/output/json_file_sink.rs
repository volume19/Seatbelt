//! JSON file output sink

use super::OutputSink;
use crate::commands::CommandDTO;
use crate::error::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Output sink that writes JSON to a file
pub struct JsonFileOutputSink {
    writer: BufWriter<File>,
    filter_results: bool,
    first_output: bool,
}

impl JsonFileOutputSink {
    /// Create a new JSON file output sink
    pub fn new(path: impl AsRef<Path>, filter_results: bool) -> Result<Self> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write opening bracket for JSON array
        writeln!(writer, "[")?;

        Ok(Self {
            writer,
            filter_results,
            first_output: true,
        })
    }
}

impl OutputSink for JsonFileOutputSink {
    fn write_output(&mut self, dto: &dyn CommandDTO) -> Result<()> {
        // Add comma separator for all but first entry
        if !self.first_output {
            writeln!(self.writer, ",")?;
        }
        self.first_output = false;

        // Serialize the DTO to JSON
        let json = serde_json::to_string(dto)?;
        write!(self.writer, "  {}", json)?;
        self.writer.flush()?;

        Ok(())
    }

    fn write_host(&mut self, _msg: &str) {
        // Host messages are not written to JSON output
    }

    fn write_error(&mut self, msg: &str) {
        eprintln!("  [!] {}", msg);
    }

    fn write_verbose(&mut self, msg: &str) {
        if !self.filter_results {
            eprintln!("  [*] {}", msg);
        }
    }

    fn write_warning(&mut self, msg: &str) {
        eprintln!("  [W] {}", msg);
    }
}

impl Drop for JsonFileOutputSink {
    fn drop(&mut self) {
        // Write closing bracket
        let _ = writeln!(self.writer, "\n]");
        let _ = self.writer.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::dto::HostDTO;
    use std::fs;

    #[test]
    fn test_json_file_sink() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.json");

        {
            let mut sink = JsonFileOutputSink::new(&file_path, false).unwrap();

            let dto1 = HostDTO::new("Message 1");
            let dto2 = HostDTO::new("Message 2");

            sink.write_output(&dto1).unwrap();
            sink.write_output(&dto2).unwrap();
        }

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.starts_with("["));
        assert!(content.ends_with("]\n"));
        assert!(content.contains("Message 1"));
        assert!(content.contains("Message 2"));
    }
}
