//! Logon Sessions command
//!
//! Enumerates active logon sessions via WMI (Win32_LogonSession class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Logon Sessions command
pub struct LogonSessionsCommand;

impl LogonSessionsCommand {
    /// Create a new LogonSessions command
    pub fn new() -> Self {
        Self
    }
}

impl Command for LogonSessionsCommand {
    fn name(&self) -> &'static str {
        "LogonSessions"
    }

    fn description(&self) -> &'static str {
        "Windows logon sessions (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::USER | CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        #[cfg(windows)]
        {
            execute_windows(runtime)
        }

        #[cfg(not(windows))]
        {
            runtime.write_warning("LogonSessions command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_LogonSession
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LogonSessionWmi {
    #[serde(rename = "LogonId")]
    logon_id: Option<String>,
    #[serde(rename = "LogonType")]
    logon_type: Option<u32>,
    #[serde(rename = "StartTime")]
    start_time: Option<String>,
    #[serde(rename = "AuthenticationPackage")]
    authentication_package: Option<String>,
}

/// Logon Session DTO
#[derive(Debug, Serialize)]
pub struct LogonSessionDTO {
    logon_id: String,
    logon_type: String,
    start_time: String,
    authentication_package: String,
}

impl CommandDTO for LogonSessionDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Logon ID:     {}\n  Logon Type:   {}\n  Start Time:   {}\n  Auth Package: {}\n",
            self.logon_id, self.logon_type, self.start_time, self.authentication_package
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<LogonSessionWmi> = wmi.query(
        "SELECT LogonId, LogonType, StartTime, AuthenticationPackage FROM Win32_LogonSession",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            let logon_type_str = match r.logon_type {
                Some(2) => "Interactive",
                Some(3) => "Network",
                Some(4) => "Batch",
                Some(5) => "Service",
                Some(7) => "Unlock",
                Some(8) => "NetworkCleartext",
                Some(9) => "NewCredentials",
                Some(10) => "RemoteInteractive",
                Some(11) => "CachedInteractive",
                _ => "Unknown",
            };

            Box::new(LogonSessionDTO {
                logon_id: r.logon_id.unwrap_or_default(),
                logon_type: logon_type_str.to_string(),
                start_time: r.start_time.unwrap_or_default(),
                authentication_package: r.authentication_package.unwrap_or_default(),
            }) as Box<dyn CommandDTO>
        })
        .collect();

    Ok(dtos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = LogonSessionsCommand::new();
        assert_eq!(cmd.name(), "LogonSessions");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
