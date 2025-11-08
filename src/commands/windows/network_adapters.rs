//! Network Adapters command
//!
//! Enumerates network adapters via WMI (Win32_NetworkAdapterConfiguration class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Network Adapters command
pub struct NetworkAdaptersCommand;

impl NetworkAdaptersCommand {
    /// Create a new NetworkAdapters command
    pub fn new() -> Self {
        Self
    }
}

impl Command for NetworkAdaptersCommand {
    fn name(&self) -> &'static str {
        "NetworkAdapters"
    }

    fn description(&self) -> &'static str {
        "Network adapters with IP addresses (via WMI)"
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
            runtime.write_warning("NetworkAdapters command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_NetworkAdapterConfiguration
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NetworkAdapterWmi {
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "MACAddress")]
    mac_address: Option<String>,
    #[serde(rename = "IPAddress")]
    ip_address: Option<Vec<String>>,
    #[serde(rename = "DHCPEnabled")]
    dhcp_enabled: Option<bool>,
    #[serde(rename = "DNSDomain")]
    dns_domain: Option<String>,
}

/// Network Adapter DTO
#[derive(Debug, Serialize)]
pub struct NetworkAdapterDTO {
    description: String,
    mac_address: String,
    ip_addresses: Vec<String>,
    dhcp_enabled: bool,
    dns_domain: String,
}

impl CommandDTO for NetworkAdapterDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        let ip_addrs = if self.ip_addresses.is_empty() {
            "None".to_string()
        } else {
            self.ip_addresses.join(", ")
        };

        format!(
            "  Description:  {}\n  MAC Address:  {}\n  IP Addresses: {}\n  DHCP Enabled: {}\n  DNS Domain:   {}\n",
            self.description, self.mac_address, ip_addrs, self.dhcp_enabled, self.dns_domain
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<NetworkAdapterWmi> = wmi.query(
        "SELECT Description, MACAddress, IPAddress, DHCPEnabled, DNSDomain FROM Win32_NetworkAdapterConfiguration WHERE IPEnabled = True",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(NetworkAdapterDTO {
                description: r.description.unwrap_or_default(),
                mac_address: r.mac_address.unwrap_or_default(),
                ip_addresses: r.ip_address.unwrap_or_default(),
                dhcp_enabled: r.dhcp_enabled.unwrap_or(false),
                dns_domain: r.dns_domain.unwrap_or_default(),
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
        let cmd = NetworkAdaptersCommand::new();
        assert_eq!(cmd.name(), "NetworkAdapters");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
