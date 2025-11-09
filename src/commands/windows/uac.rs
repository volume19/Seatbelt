//! UAC Settings command
//!
//! Checks User Account Control (UAC) configuration.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// UAC Settings command
pub struct UACCommand;

impl UACCommand {
    /// Create a new UAC command
    pub fn new() -> Self {
        Self
    }
}

impl Command for UACCommand {
    fn name(&self) -> &'static str {
        "UAC"
    }

    fn description(&self) -> &'static str {
        "User Account Control (UAC) settings"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let uac_path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System";

        let enable_lua = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "EnableLUA")?
            .unwrap_or(1);

        let consent_prompt_behavior_admin = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "ConsentPromptBehaviorAdmin")?
            .unwrap_or(5);

        let consent_prompt_behavior_user = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "ConsentPromptBehaviorUser")?
            .unwrap_or(3);

        let enable_installer_detection = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "EnableInstallerDetection")?
            .unwrap_or(1);

        let validate_admin_code_signatures = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "ValidateAdminCodeSignatures")?
            .unwrap_or(0);

        let filter_admin_token = runtime
            .registry_get_dword(RegistryHive::HKLM, uac_path, "FilterAdministratorToken")?
            .unwrap_or(0);

        let dto = UACDTO {
            enabled: enable_lua == 1,
            admin_prompt_behavior: match consent_prompt_behavior_admin {
                0 => "Elevate without prompting".to_string(),
                1 => "Prompt for credentials on the secure desktop".to_string(),
                2 => "Prompt for consent on the secure desktop".to_string(),
                3 => "Prompt for credentials".to_string(),
                4 => "Prompt for consent".to_string(),
                5 => "Prompt for consent for non-Windows binaries".to_string(),
                _ => "Unknown".to_string(),
            },
            user_prompt_behavior: match consent_prompt_behavior_user {
                0 => "Automatically deny elevation requests".to_string(),
                1 => "Prompt for credentials on the secure desktop".to_string(),
                3 => "Prompt for credentials".to_string(),
                _ => "Unknown".to_string(),
            },
            installer_detection_enabled: enable_installer_detection == 1,
            validate_signatures: validate_admin_code_signatures == 1,
            filter_admin_token: filter_admin_token == 1,
        };

        Ok(vec![Box::new(dto)])
    }
}

/// UAC DTO
#[derive(Debug, Serialize)]
pub struct UACDTO {
    enabled: bool,
    admin_prompt_behavior: String,
    user_prompt_behavior: String,
    installer_detection_enabled: bool,
    validate_signatures: bool,
    filter_admin_token: bool,
}

impl CommandDTO for UACDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  UAC Enabled:          {}\n  Admin Prompt:         {}\n  User Prompt:          {}\n  Installer Detection:  {}\n  Validate Signatures:  {}\n  Filter Admin Token:   {}\n",
            self.enabled,
            self.admin_prompt_behavior,
            self.user_prompt_behavior,
            self.installer_detection_enabled,
            self.validate_signatures,
            self.filter_admin_token
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = UACCommand::new();
        assert_eq!(cmd.name(), "UAC");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
