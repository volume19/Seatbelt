//! Processes command
//!
//! Enumerates running processes via WMI (Win32_Process class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Processes command
pub struct ProcessesCommand;

impl ProcessesCommand {
    /// Create a new Processes command
    pub fn new() -> Self {
        Self
    }
}

impl Command for ProcessesCommand {
    fn name(&self) -> &'static str {
        "Processes"
    }

    fn description(&self) -> &'static str {
        "Running processes with file info (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM | CommandGroup::MISC
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
            runtime.write_warning("Processes command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_Process
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProcessWmi {
    #[serde(rename = "ProcessId")]
    process_id: Option<u32>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "ExecutablePath")]
    executable_path: Option<String>,
    #[serde(rename = "CommandLine")]
    command_line: Option<String>,
    #[serde(rename = "SessionId")]
    session_id: Option<u32>,
}

/// Process DTO
#[derive(Debug, Serialize)]
pub struct ProcessDTO {
    process_id: u32,
    name: String,
    executable_path: String,
    command_line: String,
    session_id: u32,
}

impl CommandDTO for ProcessDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        let mut output = format!(
            "  PID:        {}\n  Name:       {}\n  Path:       {}\n",
            self.process_id, self.name, self.executable_path
        );

        if !self.command_line.is_empty() {
            output.push_str(&format!("  Command:    {}\n", self.command_line));
        }

        output.push_str(&format!("  Session ID: {}\n", self.session_id));
        output
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<ProcessWmi> = wmi.query(
        "SELECT ProcessId, Name, ExecutablePath, CommandLine, SessionId FROM Win32_Process",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(ProcessDTO {
                process_id: r.process_id.unwrap_or(0),
                name: r.name.unwrap_or_default(),
                executable_path: r.executable_path.unwrap_or_default(),
                command_line: r.command_line.unwrap_or_default(),
                session_id: r.session_id.unwrap_or(0),
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
        let cmd = ProcessesCommand::new();
        assert_eq!(cmd.name(), "Processes");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
        assert!(cmd.group().contains(CommandGroup::MISC));
    }
}
