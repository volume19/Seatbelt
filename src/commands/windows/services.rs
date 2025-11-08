//! Services command
//!
//! Enumerates Windows services via WMI (Win32_Service class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Services command
pub struct ServicesCommand;

impl ServicesCommand {
    /// Create a new Services command
    pub fn new() -> Self {
        Self
    }
}

impl Command for ServicesCommand {
    fn name(&self) -> &'static str {
        "Services"
    }

    fn description(&self) -> &'static str {
        "Services with file info (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
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
            runtime.write_warning("Services command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_Service
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ServiceWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "DisplayName")]
    display_name: Option<String>,
    #[serde(rename = "PathName")]
    path_name: Option<String>,
    #[serde(rename = "State")]
    state: Option<String>,
    #[serde(rename = "StartMode")]
    start_mode: Option<String>,
    #[serde(rename = "StartName")]
    start_name: Option<String>,
}

/// Service DTO
#[derive(Debug, Serialize)]
pub struct ServiceDTO {
    name: String,
    display_name: String,
    path_name: String,
    state: String,
    start_mode: String,
    start_name: String,
}

impl CommandDTO for ServiceDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:         {}\n  Display Name: {}\n  Path:         {}\n  State:        {}\n  Start Mode:   {}\n  Start Name:   {}\n",
            self.name,
            self.display_name,
            self.path_name,
            self.state,
            self.start_mode,
            self.start_name
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<ServiceWmi> = wmi.query(
        "SELECT Name, DisplayName, PathName, State, StartMode, StartName FROM Win32_Service",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(ServiceDTO {
                name: r.name.unwrap_or_default(),
                display_name: r.display_name.unwrap_or_default(),
                path_name: r.path_name.unwrap_or_default(),
                state: r.state.unwrap_or_default(),
                start_mode: r.start_mode.unwrap_or_default(),
                start_name: r.start_name.unwrap_or_default(),
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
        let cmd = ServicesCommand::new();
        assert_eq!(cmd.name(), "Services");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
