//! ARP Cache command
//!
//! Enumerates ARP cache entries via WMI (MSFT_NetNeighbor class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// ARP Cache command
pub struct ARPCacheCommand;

impl ARPCacheCommand {
    /// Create a new ARPCache command
    pub fn new() -> Self {
        Self
    }
}

impl Command for ARPCacheCommand {
    fn name(&self) -> &'static str {
        "ARPCache"
    }

    fn description(&self) -> &'static str {
        "ARP cache entries (via WMI)"
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
            runtime.write_warning("ARPCache command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for MSFT_NetNeighbor
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ARPEntryWmi {
    #[serde(rename = "IPAddress")]
    ip_address: Option<String>,
    #[serde(rename = "LinkLayerAddress")]
    link_layer_address: Option<String>,
    #[serde(rename = "State")]
    state: Option<u8>,
    #[serde(rename = "InterfaceAlias")]
    interface_alias: Option<String>,
}

/// ARP Cache DTO
#[derive(Debug, Serialize)]
pub struct ARPCacheDTO {
    ip_address: String,
    mac_address: String,
    state: String,
    interface: String,
}

impl CommandDTO for ARPCacheDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  IP Address:  {}\n  MAC Address: {}\n  State:       {}\n  Interface:   {}\n",
            self.ip_address, self.mac_address, self.state, self.interface
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\standardcimv2")?;
    let results: Vec<ARPEntryWmi> =
        wmi.query("SELECT IPAddress, LinkLayerAddress, State, InterfaceAlias FROM MSFT_NetNeighbor")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            let state_str = match r.state {
                Some(0) => "Unreachable",
                Some(1) => "Incomplete",
                Some(2) => "Probe",
                Some(3) => "Delay",
                Some(4) => "Stale",
                Some(5) => "Reachable",
                Some(6) => "Permanent",
                _ => "Unknown",
            };

            Box::new(ARPCacheDTO {
                ip_address: r.ip_address.unwrap_or_default(),
                mac_address: r.link_layer_address.unwrap_or_default(),
                state: state_str.to_string(),
                interface: r.interface_alias.unwrap_or_default(),
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
        let cmd = ARPCacheCommand::new();
        assert_eq!(cmd.name(), "ARPCache");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
