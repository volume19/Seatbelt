//! Recycle Bin command
//!
//! Lists files in the current user's Recycle Bin.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::path::PathBuf;

/// Recycle Bin command
pub struct RecycleBinCommand;

impl RecycleBinCommand {
    /// Create a new RecycleBin command
    pub fn new() -> Self {
        Self
    }
}

impl Command for RecycleBinCommand {
    fn name(&self) -> &'static str {
        "RecycleBin"
    }

    fn description(&self) -> &'static str {
        "Items in the Recycle Bin (last 30 days)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::USER | CommandGroup::MISC
    }

    fn supports_remote(&self) -> bool {
        false
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        #[cfg(windows)]
        {
            execute_windows(runtime)
        }

        #[cfg(not(windows))]
        {
            runtime.write_warning("RecycleBin command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// Recycle Bin Item DTO
#[derive(Debug, Serialize)]
pub struct RecycleBinItemDTO {
    file_name: String,
    original_path: String,
    size_bytes: u64,
}

impl CommandDTO for RecycleBinItemDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  File:     {}\n  Path:     {}\n  Size:     {} bytes\n",
            self.file_name, self.original_path, self.size_bytes
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    // Get the user's SID and build recycle bin path
    // For simplicity, we'll check common locations
    let mut recycle_paths = Vec::new();

    // Try to get drives and check for $Recycle.Bin
    for drive in vec!["C", "D", "E"] {
        let recycle_path = format!("{}:\\$Recycle.Bin", drive);
        if PathBuf::from(&recycle_path).exists() {
            recycle_paths.push(recycle_path);
        }
    }

    if recycle_paths.is_empty() {
        runtime.write_verbose("Could not locate Recycle Bin folders");
        return Ok(Vec::new());
    }

    let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();
    let mut total_items = 0;

    for recycle_path in recycle_paths {
        match fs::read_dir(&recycle_path) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        // This is a user SID folder
                        if let Ok(user_entries) = fs::read_dir(&path) {
                            for user_entry in user_entries.filter_map(|e| e.ok()) {
                                if let Ok(metadata) = user_entry.metadata() {
                                    total_items += 1;
                                    if total_items <= 100 {
                                        // Limit to first 100 items
                                        dtos.push(Box::new(RecycleBinItemDTO {
                                            file_name: user_entry
                                                .file_name()
                                                .to_string_lossy()
                                                .to_string(),
                                            original_path: path.to_string_lossy().to_string(),
                                            size_bytes: metadata.len(),
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // Skip if we can't read this recycle bin
            }
        }
    }

    if total_items > 100 {
        runtime.write_host(&format!(
            "  [*] Showing first 100 of {} total items",
            total_items
        ));
    }

    Ok(dtos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = RecycleBinCommand::new();
        assert_eq!(cmd.name(), "RecycleBin");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::MISC));
    }
}
