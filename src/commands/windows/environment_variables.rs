//! Environment Variables command
//!
//! Enumerates all environment variables.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;
use std::env;

/// Environment Variables command
pub struct EnvironmentVariablesCommand;

impl EnvironmentVariablesCommand {
    /// Create a new EnvironmentVariables command
    pub fn new() -> Self {
        Self
    }
}

impl Command for EnvironmentVariablesCommand {
    fn name(&self) -> &'static str {
        "EnvironmentVariables"
    }

    fn description(&self) -> &'static str {
        "Current environment variables"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        false
    }

    fn execute(&self, _runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut env_vars: Vec<(String, String)> = env::vars().collect();

        // Sort by name for consistent output
        env_vars.sort_by(|a, b| a.0.cmp(&b.0));

        let dtos: Vec<Box<dyn CommandDTO>> = env_vars
            .into_iter()
            .map(|(name, value)| Box::new(EnvironmentVariableDTO { name, value }) as Box<dyn CommandDTO>)
            .collect();

        Ok(dtos)
    }
}

/// Environment Variable DTO
#[derive(Debug, Serialize)]
pub struct EnvironmentVariableDTO {
    name: String,
    value: String,
}

impl CommandDTO for EnvironmentVariableDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!("  {}={}\n", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = EnvironmentVariablesCommand::new();
        assert_eq!(cmd.name(), "EnvironmentVariables");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
