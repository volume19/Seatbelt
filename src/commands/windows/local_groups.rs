//! Local Groups command
//!
//! Enumerates local groups and their members via WMI.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Local Groups command
pub struct LocalGroupsCommand;

impl LocalGroupsCommand {
    /// Create a new LocalGroups command
    pub fn new() -> Self {
        Self
    }
}

impl Command for LocalGroupsCommand {
    fn name(&self) -> &'static str {
        "LocalGroups"
    }

    fn description(&self) -> &'static str {
        "Local groups (via WMI)"
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
            runtime.write_warning("LocalGroups command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_Group
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GroupWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Domain")]
    domain: Option<String>,
    #[serde(rename = "SID")]
    sid: Option<String>,
    #[serde(rename = "LocalAccount")]
    local_account: Option<bool>,
}

/// Local Group DTO
#[derive(Debug, Serialize)]
pub struct LocalGroupDTO {
    name: String,
    domain: String,
    sid: String,
    local_account: bool,
}

impl CommandDTO for LocalGroupDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:          {}\n  Domain:        {}\n  SID:           {}\n  Local Account: {}\n",
            self.name, self.domain, self.sid, self.local_account
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<GroupWmi> = wmi.query(
        "SELECT Name, Domain, SID, LocalAccount FROM Win32_Group WHERE LocalAccount = True",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(LocalGroupDTO {
                name: r.name.unwrap_or_default(),
                domain: r.domain.unwrap_or_default(),
                sid: r.sid.unwrap_or_default(),
                local_account: r.local_account.unwrap_or(false),
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
        let cmd = LocalGroupsCommand::new();
        assert_eq!(cmd.name(), "LocalGroups");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
