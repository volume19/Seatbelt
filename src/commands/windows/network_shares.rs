//! Network Shares command
//!
//! Enumerates network shares via WMI (Win32_Share class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Network Shares command
pub struct NetworkSharesCommand;

impl NetworkSharesCommand {
    /// Create a new NetworkShares command
    pub fn new() -> Self {
        Self
    }
}

impl Command for NetworkSharesCommand {
    fn name(&self) -> &'static str {
        "NetworkShares"
    }

    fn description(&self) -> &'static str {
        "Network shares exposed by the machine (via WMI)"
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
            runtime.write_warning("NetworkShares command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_Share
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ShareWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Path")]
    path: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
}

/// Network Share DTO
#[derive(Debug, Serialize)]
pub struct NetworkShareDTO {
    name: String,
    path: String,
    description: String,
}

impl CommandDTO for NetworkShareDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:        {}\n  Path:        {}\n  Description: {}\n",
            self.name, self.path, self.description
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<ShareWmi> = wmi.query("SELECT Name, Path, Description FROM Win32_Share")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(NetworkShareDTO {
                name: r.name.unwrap_or_default(),
                path: r.path.unwrap_or_default(),
                description: r.description.unwrap_or_default(),
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
        let cmd = NetworkSharesCommand::new();
        assert_eq!(cmd.name(), "NetworkShares");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
