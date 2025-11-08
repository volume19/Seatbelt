//! PowerShell command
//!
//! Checks PowerShell versions and settings via registry.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// PowerShell command
pub struct PowerShellCommand;

impl PowerShellCommand {
    /// Create a new PowerShell command
    pub fn new() -> Self {
        Self
    }
}

impl Command for PowerShellCommand {
    fn name(&self) -> &'static str {
        "PowerShell"
    }

    fn description(&self) -> &'static str {
        "PowerShell versions and security settings"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Check installed PowerShell versions
        let ps_versions = vec!["1", "2", "3", "4", "5"];

        for version in ps_versions {
            let key_path = format!(
                "SOFTWARE\\Microsoft\\PowerShell\\{}\\PowerShellEngine",
                version
            );
            if let Ok(Some(ps_version)) =
                runtime.registry_get_string(RegistryHive::HKLM, &key_path, "PowerShellVersion")
            {
                let install_path = runtime
                    .registry_get_string(RegistryHive::HKLM, &key_path, "ApplicationBase")?
                    .unwrap_or_default();

                dtos.push(Box::new(PowerShellVersionDTO {
                    major_version: version.to_string(),
                    powershell_version: ps_version,
                    install_path,
                }));
            }
        }

        // Check execution policy settings
        let execution_policies = vec![
            ("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell", "Machine Policy"),
            ("HKCU\\SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell", "User Policy"),
            ("HKLM\\SOFTWARE\\Microsoft\\PowerShell\\1\\ShellIds\\Microsoft.PowerShell", "Machine Default"),
        ];

        for (path, scope) in execution_policies {
            let (hive, key_path) = if path.starts_with("HKLM\\") {
                (RegistryHive::HKLM, &path[5..])
            } else {
                (RegistryHive::HKCU, &path[5..])
            };

            if let Ok(Some(policy)) =
                runtime.registry_get_string(hive, key_path, "ExecutionPolicy")
            {
                dtos.push(Box::new(PowerShellPolicyDTO {
                    scope: scope.to_string(),
                    execution_policy: policy,
                }));
            }
        }

        // Check transcription settings
        let transcription_path =
            "SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell\\Transcription";
        let transcription_enabled = runtime
            .registry_get_dword(RegistryHive::HKLM, transcription_path, "EnableTranscripting")?
            .unwrap_or(0);

        if transcription_enabled == 1 {
            let output_directory = runtime
                .registry_get_string(RegistryHive::HKLM, transcription_path, "OutputDirectory")?
                .unwrap_or_default();

            dtos.push(Box::new(PowerShellTranscriptionDTO {
                enabled: true,
                output_directory,
            }));
        }

        // Check script block logging
        let scriptblock_path =
            "SOFTWARE\\Policies\\Microsoft\\Windows\\PowerShell\\ScriptBlockLogging";
        let scriptblock_enabled = runtime
            .registry_get_dword(RegistryHive::HKLM, scriptblock_path, "EnableScriptBlockLogging")?
            .unwrap_or(0);

        if scriptblock_enabled == 1 {
            dtos.push(Box::new(PowerShellScriptBlockLoggingDTO {
                enabled: true,
            }));
        }

        Ok(dtos)
    }
}

/// PowerShell version DTO
#[derive(Debug, Serialize)]
pub struct PowerShellVersionDTO {
    major_version: String,
    powershell_version: String,
    install_path: String,
}

impl CommandDTO for PowerShellVersionDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  PowerShell v{}\n    Version:      {}\n    Install Path: {}\n",
            self.major_version, self.powershell_version, self.install_path
        )
    }
}

/// PowerShell execution policy DTO
#[derive(Debug, Serialize)]
pub struct PowerShellPolicyDTO {
    scope: String,
    execution_policy: String,
}

impl CommandDTO for PowerShellPolicyDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Execution Policy ({}): {}\n",
            self.scope, self.execution_policy
        )
    }
}

/// PowerShell transcription DTO
#[derive(Debug, Serialize)]
pub struct PowerShellTranscriptionDTO {
    enabled: bool,
    output_directory: String,
}

impl CommandDTO for PowerShellTranscriptionDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Transcription Enabled: {}\n  Output Directory:      {}\n",
            self.enabled, self.output_directory
        )
    }
}

/// PowerShell script block logging DTO
#[derive(Debug, Serialize)]
pub struct PowerShellScriptBlockLoggingDTO {
    enabled: bool,
}

impl CommandDTO for PowerShellScriptBlockLoggingDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!("  Script Block Logging:  {}\n", self.enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = PowerShellCommand::new();
        assert_eq!(cmd.name(), "PowerShell");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
