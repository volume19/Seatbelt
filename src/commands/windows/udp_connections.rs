//! UDP Connections command
//!
//! Enumerates active UDP endpoints via WMI.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// UDP Connections command
pub struct UDPConnectionsCommand;

impl UDPConnectionsCommand {
    /// Create a new UDPConnections command
    pub fn new() -> Self {
        Self
    }
}

impl Command for UDPConnectionsCommand {
    fn name(&self) -> &'static str {
        "UDPConnections"
    }

    fn description(&self) -> &'static str {
        "Active UDP endpoints (via WMI)"
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
            runtime.write_warning("UDPConnections command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for MSFT_NetUDPEndpoint
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UDPEndpointWmi {
    #[serde(rename = "LocalAddress")]
    local_address: Option<String>,
    #[serde(rename = "LocalPort")]
    local_port: Option<u16>,
    #[serde(rename = "OwningProcess")]
    owning_process: Option<u32>,
}

/// UDP Endpoint DTO
#[derive(Debug, Serialize)]
pub struct UDPEndpointDTO {
    local_address: String,
    local_port: u16,
    process_id: u32,
}

impl CommandDTO for UDPEndpointDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Local:   {}:{}\n  PID:     {}\n",
            self.local_address, self.local_port, self.process_id
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\standardcimv2")?;
    let results: Vec<UDPEndpointWmi> =
        wmi.query("SELECT LocalAddress, LocalPort, OwningProcess FROM MSFT_NetUDPEndpoint")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(UDPEndpointDTO {
                local_address: r.local_address.unwrap_or_default(),
                local_port: r.local_port.unwrap_or(0),
                process_id: r.owning_process.unwrap_or(0),
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
        let cmd = UDPConnectionsCommand::new();
        assert_eq!(cmd.name(), "UDPConnections");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
