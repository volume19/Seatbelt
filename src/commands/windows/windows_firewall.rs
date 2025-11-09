//! Windows Firewall command
//!
//! Checks Windows Firewall settings via registry.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Windows Firewall command
pub struct WindowsFirewallCommand;

impl WindowsFirewallCommand {
    /// Create a new WindowsFirewall command
    pub fn new() -> Self {
        Self
    }
}

impl Command for WindowsFirewallCommand {
    fn name(&self) -> &'static str {
        "WindowsFirewall"
    }

    fn description(&self) -> &'static str {
        "Windows Firewall settings"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Check firewall profiles: Domain, Standard (Private), Public
        let profiles = vec![
            ("DomainProfile", "Domain"),
            ("StandardProfile", "Private"),
            ("PublicProfile", "Public"),
        ];

        for (reg_name, profile_name) in profiles {
            let base_path = format!(
                "SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\{}",
                reg_name
            );

            let enabled = runtime
                .registry_get_dword(RegistryHive::HKLM, &base_path, "EnableFirewall")?
                .unwrap_or(0);

            let default_inbound_action = runtime
                .registry_get_dword(RegistryHive::HKLM, &base_path, "DefaultInboundAction")?
                .unwrap_or(0);

            let default_outbound_action = runtime
                .registry_get_dword(RegistryHive::HKLM, &base_path, "DefaultOutboundAction")?
                .unwrap_or(0);

            let disable_notifications = runtime
                .registry_get_dword(RegistryHive::HKLM, &base_path, "DisableNotifications")?
                .unwrap_or(0);

            dtos.push(Box::new(WindowsFirewallProfileDTO {
                profile: profile_name.to_string(),
                enabled: enabled == 1,
                default_inbound_action: if default_inbound_action == 0 {
                    "Allow".to_string()
                } else {
                    "Block".to_string()
                },
                default_outbound_action: if default_outbound_action == 0 {
                    "Allow".to_string()
                } else {
                    "Block".to_string()
                },
                notifications_disabled: disable_notifications == 1,
            }));
        }

        Ok(dtos)
    }
}

/// Windows Firewall profile DTO
#[derive(Debug, Serialize)]
pub struct WindowsFirewallProfileDTO {
    profile: String,
    enabled: bool,
    default_inbound_action: String,
    default_outbound_action: String,
    notifications_disabled: bool,
}

impl CommandDTO for WindowsFirewallProfileDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Profile:             {}\n  Enabled:             {}\n  Default Inbound:     {}\n  Default Outbound:    {}\n  Notifications Off:   {}\n",
            self.profile,
            self.enabled,
            self.default_inbound_action,
            self.default_outbound_action,
            self.notifications_disabled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = WindowsFirewallCommand::new();
        assert_eq!(cmd.name(), "WindowsFirewall");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
