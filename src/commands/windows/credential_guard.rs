//! Credential Guard command
//!
//! Checks Credential Guard and Device Guard status.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Credential Guard command
pub struct CredentialGuardCommand;

impl CredentialGuardCommand {
    /// Create a new CredentialGuard command
    pub fn new() -> Self {
        Self
    }
}

impl Command for CredentialGuardCommand {
    fn name(&self) -> &'static str {
        "CredentialGuard"
    }

    fn description(&self) -> &'static str {
        "Credential Guard configuration"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        // Check Device Guard/Credential Guard settings
        let dg_path = "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard";

        let enable_virtualization_based_security = runtime
            .registry_get_dword(RegistryHive::HKLM, dg_path, "EnableVirtualizationBasedSecurity")?
            .unwrap_or(0);

        let require_platform_security_features = runtime
            .registry_get_dword(RegistryHive::HKLM, dg_path, "RequirePlatformSecurityFeatures")?
            .unwrap_or(0);

        // Credential Guard specific
        let cg_path = "SYSTEM\\CurrentControlSet\\Control\\Lsa";
        let lsa_cfg_flags = runtime
            .registry_get_dword(RegistryHive::HKLM, cg_path, "LsaCfgFlags")?
            .unwrap_or(0);

        let dto = CredentialGuardDTO {
            virtualization_based_security_enabled: enable_virtualization_based_security == 1,
            platform_security_level: match require_platform_security_features {
                1 => "Secure Boot".to_string(),
                3 => "Secure Boot and DMA Protection".to_string(),
                _ => "Not configured".to_string(),
            },
            credential_guard_enabled: lsa_cfg_flags == 1 || lsa_cfg_flags == 2,
            credential_guard_configuration: match lsa_cfg_flags {
                0 => "Disabled".to_string(),
                1 => "Enabled with UEFI lock".to_string(),
                2 => "Enabled without lock".to_string(),
                _ => "Unknown".to_string(),
            },
        };

        Ok(vec![Box::new(dto)])
    }
}

/// Credential Guard DTO
#[derive(Debug, Serialize)]
pub struct CredentialGuardDTO {
    virtualization_based_security_enabled: bool,
    platform_security_level: String,
    credential_guard_enabled: bool,
    credential_guard_configuration: String,
}

impl CommandDTO for CredentialGuardDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  VBS Enabled:              {}\n  Platform Security:        {}\n  Credential Guard Enabled: {}\n  Credential Guard Config:  {}\n",
            self.virtualization_based_security_enabled,
            self.platform_security_level,
            self.credential_guard_enabled,
            self.credential_guard_configuration
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = CredentialGuardCommand::new();
        assert_eq!(cmd.name(), "CredentialGuard");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
