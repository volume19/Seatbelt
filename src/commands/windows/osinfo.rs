//! OS Info command
//!
//! Enumerates basic OS information via WMI (Win32_OperatingSystem class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// OS Info command
pub struct OSInfoCommand;

impl OSInfoCommand {
    /// Create a new OSInfo command
    pub fn new() -> Self {
        Self
    }
}

impl Command for OSInfoCommand {
    fn name(&self) -> &'static str {
        "OSInfo"
    }

    fn description(&self) -> &'static str {
        "Basic OS information (i.e. architecture, OS version, etc.)"
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
            runtime.write_warning("OSInfo command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_OperatingSystem
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OSInfoWmi {
    #[serde(rename = "Caption")]
    caption: Option<String>,
    #[serde(rename = "Version")]
    version: Option<String>,
    #[serde(rename = "BuildNumber")]
    build_number: Option<String>,
    #[serde(rename = "OSArchitecture")]
    os_architecture: Option<String>,
    #[serde(rename = "InstallDate")]
    install_date: Option<String>,
    #[serde(rename = "LastBootUpTime")]
    last_boot_up_time: Option<String>,
    #[serde(rename = "SystemDirectory")]
    system_directory: Option<String>,
    #[serde(rename = "NumberOfUsers")]
    number_of_users: Option<u32>,
}

/// OS Info DTO
#[derive(Debug, Serialize)]
pub struct OSInfoDTO {
    caption: String,
    version: String,
    build_number: String,
    os_architecture: String,
    install_date: String,
    last_boot_up_time: String,
    system_directory: String,
    number_of_users: u32,
}

impl CommandDTO for OSInfoDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Caption:            {}\n  Version:            {}\n  Build Number:       {}\n  Architecture:       {}\n  Install Date:       {}\n  Last Boot:          {}\n  System Directory:   {}\n  Number of Users:    {}\n",
            self.caption,
            self.version,
            self.build_number,
            self.os_architecture,
            self.install_date,
            self.last_boot_up_time,
            self.system_directory,
            self.number_of_users
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<OSInfoWmi> = wmi.query(
        "SELECT Caption, Version, BuildNumber, OSArchitecture, InstallDate, LastBootUpTime, SystemDirectory, NumberOfUsers FROM Win32_OperatingSystem",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(OSInfoDTO {
                caption: r.caption.unwrap_or_default(),
                version: r.version.unwrap_or_default(),
                build_number: r.build_number.unwrap_or_default(),
                os_architecture: r.os_architecture.unwrap_or_default(),
                install_date: r.install_date.unwrap_or_default(),
                last_boot_up_time: r.last_boot_up_time.unwrap_or_default(),
                system_directory: r.system_directory.unwrap_or_default(),
                number_of_users: r.number_of_users.unwrap_or(0),
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
        let cmd = OSInfoCommand::new();
        assert_eq!(cmd.name(), "OSInfo");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
