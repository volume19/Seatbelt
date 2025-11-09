//! RDP Settings command
//!
//! Checks Remote Desktop Protocol (RDP) configuration.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// RDP Settings command
pub struct RDPSettingsCommand;

impl RDPSettingsCommand {
    /// Create a new RDPSettings command
    pub fn new() -> Self {
        Self
    }
}

impl Command for RDPSettingsCommand {
    fn name(&self) -> &'static str {
        "RDPSettings"
    }

    fn description(&self) -> &'static str {
        "Remote Desktop Protocol (RDP) settings"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let rdp_path = "SYSTEM\\CurrentControlSet\\Control\\Terminal Server";

        // Check if RDP is enabled
        let deny_ts_connections = runtime
            .registry_get_dword(RegistryHive::HKLM, rdp_path, "fDenyTSConnections")?
            .unwrap_or(1);

        // Network Level Authentication
        let user_authentication = runtime
            .registry_get_dword(
                RegistryHive::HKLM,
                &format!("{}\\WinStations\\RDP-Tcp", rdp_path),
                "UserAuthentication",
            )?
            .unwrap_or(0);

        let security_layer = runtime
            .registry_get_dword(
                RegistryHive::HKLM,
                &format!("{}\\WinStations\\RDP-Tcp", rdp_path),
                "SecurityLayer",
            )?
            .unwrap_or(0);

        // RDP port
        let port_number = runtime
            .registry_get_dword(
                RegistryHive::HKLM,
                &format!("{}\\WinStations\\RDP-Tcp", rdp_path),
                "PortNumber",
            )?
            .unwrap_or(3389);

        let dto = RDPSettingsDTO {
            enabled: deny_ts_connections == 0,
            nla_enabled: user_authentication == 1,
            security_layer: match security_layer {
                0 => "RDP Security Layer".to_string(),
                1 => "Negotiate".to_string(),
                2 => "SSL/TLS".to_string(),
                _ => "Unknown".to_string(),
            },
            port: port_number,
        };

        Ok(vec![Box::new(dto)])
    }
}

/// RDP Settings DTO
#[derive(Debug, Serialize)]
pub struct RDPSettingsDTO {
    enabled: bool,
    nla_enabled: bool,
    security_layer: String,
    port: u32,
}

impl CommandDTO for RDPSettingsDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  RDP Enabled:      {}\n  NLA Enabled:      {}\n  Security Layer:   {}\n  Port:             {}\n",
            self.enabled, self.nla_enabled, self.security_layer, self.port
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = RDPSettingsCommand::new();
        assert_eq!(cmd.name(), "RDPSettings");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
