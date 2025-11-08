//! Hotfixes command
//!
//! Enumerates installed Windows updates/hotfixes via WMI (Win32_QuickFixEngineering class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Hotfixes command
pub struct HotfixesCommand;

impl HotfixesCommand {
    /// Create a new Hotfixes command
    pub fn new() -> Self {
        Self
    }
}

impl Command for HotfixesCommand {
    fn name(&self) -> &'static str {
        "Hotfixes"
    }

    fn description(&self) -> &'static str {
        "Installed hotfixes (via WMI)"
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
            runtime.write_warning("Hotfixes command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_QuickFixEngineering
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HotfixWmi {
    #[serde(rename = "HotFixID")]
    hotfix_id: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "InstalledBy")]
    installed_by: Option<String>,
    #[serde(rename = "InstalledOn")]
    installed_on: Option<String>,
}

/// Hotfix DTO
#[derive(Debug, Serialize)]
pub struct HotfixDTO {
    hotfix_id: String,
    description: String,
    installed_by: String,
    installed_on: String,
}

impl CommandDTO for HotfixDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  HotFixID:     {}\n  Description:  {}\n  Installed By: {}\n  Installed On: {}\n",
            self.hotfix_id, self.description, self.installed_by, self.installed_on
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<HotfixWmi> =
        wmi.query("SELECT HotFixID, Description, InstalledBy, InstalledOn FROM Win32_QuickFixEngineering")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(HotfixDTO {
                hotfix_id: r.hotfix_id.unwrap_or_default(),
                description: r.description.unwrap_or_default(),
                installed_by: r.installed_by.unwrap_or_default(),
                installed_on: r.installed_on.unwrap_or_default(),
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
        let cmd = HotfixesCommand::new();
        assert_eq!(cmd.name(), "Hotfixes");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
