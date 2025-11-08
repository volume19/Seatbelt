//! DNS Cache command
//!
//! Lists DNS cache entries via WMI (MSFT_DNSClientCache class).
//! Requires Windows 8/Server 2012 or later.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::{Result, SeatbeltError};
use crate::runtime::Runtime;
use serde::{Deserialize, Serialize};

/// DNS Cache command
pub struct DnsCacheCommand;

impl DnsCacheCommand {
    /// Create a new DNSCache command
    pub fn new() -> Self {
        Self
    }
}

impl Command for DnsCacheCommand {
    fn name(&self) -> &'static str {
        "DNSCache"
    }

    fn description(&self) -> &'static str {
        "DNS cache entries (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM | CommandGroup::REMOTE
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
            runtime.write_warning("DNSCache command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for MSFT_DNSClientCache
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DnsCacheWmi {
    #[serde(rename = "Entry")]
    entry: Option<String>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Data")]
    data: Option<String>,
}

/// DNS Cache DTO
#[derive(Debug, Serialize)]
pub struct DnsCacheDTO {
    entry: String,
    name: String,
    data: String,
}

impl CommandDTO for DnsCacheDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Entry: {}\n  Name:  {}\n  Data:  {}\n",
            self.entry, self.name, self.data
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\standardcimv2").map_err(|e| {
        SeatbeltError::WmiError(format!(
            "'MSFT_DNSClientCache' WMI class unavailable (minimum supported versions of Windows: 8/2012): {}",
            e
        ))
    })?;

    let results: Vec<DnsCacheWmi> = wmi.query("SELECT Entry, Name, Data FROM MSFT_DNSClientCache")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(DnsCacheDTO {
                entry: r.entry.unwrap_or_else(|| "".to_string()),
                name: r.name.unwrap_or_else(|| "".to_string()),
                data: r.data.unwrap_or_else(|| "".to_string()),
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
        let cmd = DnsCacheCommand::new();
        assert_eq!(cmd.name(), "DNSCache");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
        assert!(cmd.group().contains(CommandGroup::REMOTE));
    }

    #[test]
    fn test_dto_serialization() {
        let dto = DnsCacheDTO {
            entry: "test.com".to_string(),
            name: "test".to_string(),
            data: "192.168.1.1".to_string(),
        };

        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("test.com"));
        assert!(json.contains("192.168.1.1"));
    }
}
