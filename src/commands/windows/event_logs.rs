//! Event Logs command
//!
//! Checks Windows Event Log configuration.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Event Logs command
pub struct EventLogsCommand;

impl EventLogsCommand {
    /// Create a new EventLogs command
    pub fn new() -> Self {
        Self
    }
}

impl Command for EventLogsCommand {
    fn name(&self) -> &'static str {
        "EventLogs"
    }

    fn description(&self) -> &'static str {
        "Event log settings and retention policies"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Common event logs to check
        let log_names = vec![
            "Application",
            "Security",
            "System",
            "Microsoft-Windows-PowerShell/Operational",
            "Microsoft-Windows-Sysmon/Operational",
        ];

        for log_name in log_names {
            let log_path = format!("SYSTEM\\CurrentControlSet\\Services\\EventLog\\{}", log_name);

            // Check if log exists
            let max_size = runtime
                .registry_get_dword(RegistryHive::HKLM, &log_path, "MaxSize")?;

            if let Some(max_size_kb) = max_size {
                let retention = runtime
                    .registry_get_dword(RegistryHive::HKLM, &log_path, "Retention")?
                    .unwrap_or(0);

                let auto_backup = runtime
                    .registry_get_dword(RegistryHive::HKLM, &log_path, "AutoBackupLogFiles")?
                    .unwrap_or(0);

                dtos.push(Box::new(EventLogDTO {
                    log_name: log_name.to_string(),
                    max_size_kb,
                    retention_days: retention / 86400, // Convert seconds to days
                    auto_backup_enabled: auto_backup == 1,
                }));
            }
        }

        if dtos.is_empty() {
            runtime.write_host("  [*] Could not enumerate event log settings");
        }

        Ok(dtos)
    }
}

/// Event Log DTO
#[derive(Debug, Serialize)]
pub struct EventLogDTO {
    log_name: String,
    max_size_kb: u32,
    retention_days: u32,
    auto_backup_enabled: bool,
}

impl CommandDTO for EventLogDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Log:           {}\n  Max Size:      {} KB\n  Retention:     {} days\n  Auto Backup:   {}\n",
            self.log_name, self.max_size_kb, self.retention_days, self.auto_backup_enabled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = EventLogsCommand::new();
        assert_eq!(cmd.name(), "EventLogs");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
