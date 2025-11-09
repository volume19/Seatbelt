//! TCP Connections command
//!
//! Enumerates active TCP connections via WMI.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// TCP Connections command
pub struct TCPConnectionsCommand;

impl TCPConnectionsCommand {
    /// Create a new TCPConnections command
    pub fn new() -> Self {
        Self
    }
}

impl Command for TCPConnectionsCommand {
    fn name(&self) -> &'static str {
        "TCPConnections"
    }

    fn description(&self) -> &'static str {
        "Active TCP connections (via WMI)"
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
            runtime.write_warning("TCPConnections command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for MSFT_NetTCPConnection
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TCPConnectionWmi {
    #[serde(rename = "LocalAddress")]
    local_address: Option<String>,
    #[serde(rename = "LocalPort")]
    local_port: Option<u16>,
    #[serde(rename = "RemoteAddress")]
    remote_address: Option<String>,
    #[serde(rename = "RemotePort")]
    remote_port: Option<u16>,
    #[serde(rename = "State")]
    state: Option<u8>,
    #[serde(rename = "OwningProcess")]
    owning_process: Option<u32>,
}

/// TCP Connection DTO
#[derive(Debug, Serialize)]
pub struct TCPConnectionDTO {
    local_address: String,
    local_port: u16,
    remote_address: String,
    remote_port: u16,
    state: String,
    process_id: u32,
}

impl CommandDTO for TCPConnectionDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Local:   {}:{}\n  Remote:  {}:{}\n  State:   {}\n  PID:     {}\n",
            self.local_address,
            self.local_port,
            self.remote_address,
            self.remote_port,
            self.state,
            self.process_id
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\standardcimv2")?;
    let results: Vec<TCPConnectionWmi> = wmi.query(
        "SELECT LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess FROM MSFT_NetTCPConnection",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            let state_str = match r.state {
                Some(1) => "Closed",
                Some(2) => "Listen",
                Some(3) => "SynSent",
                Some(4) => "SynReceived",
                Some(5) => "Established",
                Some(6) => "FinWait1",
                Some(7) => "FinWait2",
                Some(8) => "CloseWait",
                Some(9) => "Closing",
                Some(10) => "LastAck",
                Some(11) => "TimeWait",
                Some(12) => "DeleteTCB",
                _ => "Unknown",
            };

            Box::new(TCPConnectionDTO {
                local_address: r.local_address.unwrap_or_default(),
                local_port: r.local_port.unwrap_or(0),
                remote_address: r.remote_address.unwrap_or_default(),
                remote_port: r.remote_port.unwrap_or(0),
                state: state_str.to_string(),
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
        let cmd = TCPConnectionsCommand::new();
        assert_eq!(cmd.name(), "TCPConnections");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
