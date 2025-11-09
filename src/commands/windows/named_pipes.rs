//! Named Pipes command
//!
//! Enumerates named pipes on the system.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use std::fs;

/// Named Pipes command
pub struct NamedPipesCommand;

impl NamedPipesCommand {
    /// Create a new NamedPipes command
    pub fn new() -> Self {
        Self
    }
}

impl Command for NamedPipesCommand {
    fn name(&self) -> &'static str {
        "NamedPipes"
    }

    fn description(&self) -> &'static str {
        "Named pipes"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
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
            runtime.write_warning("NamedPipes command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// Named Pipe DTO
#[derive(Debug, Serialize)]
pub struct NamedPipeDTO {
    pipe_name: String,
}

impl CommandDTO for NamedPipeDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!("  {}\n", self.pipe_name)
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let pipe_path = r"\\.\pipe\";

    match fs::read_dir(pipe_path) {
        Ok(entries) => {
            let dtos: Vec<Box<dyn CommandDTO>> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| {
                    Box::new(NamedPipeDTO {
                        pipe_name: entry.file_name().to_string_lossy().to_string(),
                    }) as Box<dyn CommandDTO>
                })
                .collect();
            Ok(dtos)
        }
        Err(e) => {
            runtime.write_error(&format!("Failed to enumerate named pipes: {}", e));
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = NamedPipesCommand::new();
        assert_eq!(cmd.name(), "NamedPipes");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
