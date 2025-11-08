//! Environment PATH command
//!
//! Enumerates directories in the system PATH environment variable.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;
use std::env;

/// Environment PATH command
pub struct EnvironmentPathCommand;

impl EnvironmentPathCommand {
    /// Create a new EnvironmentPath command
    pub fn new() -> Self {
        Self
    }
}

impl Command for EnvironmentPathCommand {
    fn name(&self) -> &'static str {
        "EnvironmentPath"
    }

    fn description(&self) -> &'static str {
        "Current environment %PATH$ folders"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        false
    }

    fn execute(&self, _runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let path_var = env::var("PATH").unwrap_or_default();

        #[cfg(windows)]
        let separator = ';';
        #[cfg(not(windows))]
        let separator = ':';

        let paths: Vec<String> = path_var
            .split(separator)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let dtos: Vec<Box<dyn CommandDTO>> = paths
            .into_iter()
            .map(|path| Box::new(EnvironmentPathDTO { path }) as Box<dyn CommandDTO>)
            .collect();

        Ok(dtos)
    }
}

/// Environment PATH entry DTO
#[derive(Debug, Serialize)]
pub struct EnvironmentPathDTO {
    path: String,
}

impl CommandDTO for EnvironmentPathDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!("  {}\n", self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = EnvironmentPathCommand::new();
        assert_eq!(cmd.name(), "EnvironmentPath");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
