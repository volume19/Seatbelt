//! Secure Boot command
//!
//! Checks if Secure Boot is enabled via registry.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Secure Boot command
pub struct SecureBootCommand;

impl SecureBootCommand {
    /// Create a new SecureBoot command
    pub fn new() -> Self {
        Self
    }
}

impl Command for SecureBootCommand {
    fn name(&self) -> &'static str {
        "SecureBoot"
    }

    fn description(&self) -> &'static str {
        "Secure Boot configuration"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let secure_boot_enabled = runtime
            .registry_get_dword(
                RegistryHive::HKLM,
                "SYSTEM\\CurrentControlSet\\Control\\SecureBoot\\State",
                "UEFISecureBootEnabled",
            )?
            .unwrap_or(0);

        let dto = SecureBootDTO {
            secure_boot_enabled: secure_boot_enabled == 1,
            uefi_secure_boot_value: secure_boot_enabled,
        };

        Ok(vec![Box::new(dto)])
    }
}

/// Secure Boot configuration DTO
#[derive(Debug, Serialize)]
pub struct SecureBootDTO {
    secure_boot_enabled: bool,
    uefi_secure_boot_value: u32,
}

impl CommandDTO for SecureBootDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Secure Boot Enabled:      {}\n  UEFI Secure Boot Value:   {}\n",
            self.secure_boot_enabled, self.uefi_secure_boot_value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = SecureBootCommand::new();
        assert_eq!(cmd.name(), "SecureBoot");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
