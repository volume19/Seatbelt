//! File output writer

use super::TextWriter;
use crate::error::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Writer that outputs to a file
pub struct FileWriter {
    writer: BufWriter<File>,
}

impl FileWriter {
    /// Create a new file writer
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }
}

impl TextWriter for FileWriter {
    fn write(&mut self, text: &str) -> Result<()> {
        self.writer.write_all(text.as_bytes())?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        let _ = self.writer.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_writer() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        {
            let mut writer = FileWriter::new(&file_path).unwrap();
            writer.write("Hello\n").unwrap();
            writer.write("World\n").unwrap();
            writer.flush().unwrap();
        }

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello\nWorld\n");
    }
}
