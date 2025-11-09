//! AutoRuns command
//!
//! Checks common autorun/persistence locations via registry.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// AutoRuns command
pub struct AutoRunsCommand;

impl AutoRunsCommand {
    /// Create a new AutoRuns command
    pub fn new() -> Self {
        Self
    }
}

impl Command for AutoRunsCommand {
    fn name(&self) -> &'static str {
        "AutoRuns"
    }

    fn description(&self) -> &'static str {
        "Auto-run executables/scripts/programs from common locations"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Common autorun registry locations
        let locations = vec![
            (RegistryHive::HKLM, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", "HKLM Run"),
            (RegistryHive::HKLM, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce", "HKLM RunOnce"),
            (RegistryHive::HKCU, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", "HKCU Run"),
            (RegistryHive::HKCU, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce", "HKCU RunOnce"),
            (RegistryHive::HKLM, "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Run", "HKLM Run (Wow6432Node)"),
            (RegistryHive::HKLM, "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\RunOnce", "HKLM RunOnce (Wow6432Node)"),
        ];

        for (hive, path, location_name) in locations {
            match runtime.registry_get_values(hive, path) {
                Ok(values) => {
                    for (name, value) in values {
                        dtos.push(Box::new(AutoRunDTO {
                            location: location_name.to_string(),
                            name,
                            value,
                        }));
                    }
                }
                Err(_) => {
                    // Key doesn't exist, skip silently
                }
            }
        }

        // Check startup folders (via registry - just list the paths, not the actual files)
        let startup_folders = vec![
            (RegistryHive::HKCU, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders", "Startup", "User Startup"),
            (RegistryHive::HKLM, "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders", "Common Startup", "Common Startup"),
        ];

        for (hive, path, value_name, location_name) in startup_folders {
            if let Ok(Some(folder_path)) = runtime.registry_get_string(hive, path, value_name) {
                dtos.push(Box::new(AutoRunDTO {
                    location: location_name.to_string(),
                    name: "Folder".to_string(),
                    value: folder_path,
                }));
            }
        }

        Ok(dtos)
    }
}

/// AutoRun DTO
#[derive(Debug, Serialize)]
pub struct AutoRunDTO {
    location: String,
    name: String,
    value: String,
}

impl CommandDTO for AutoRunDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Location: {}\n  Name:     {}\n  Value:    {}\n",
            self.location, self.name, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = AutoRunsCommand::new();
        assert_eq!(cmd.name(), "AutoRuns");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
