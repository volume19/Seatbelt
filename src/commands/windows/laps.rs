//! LAPS command
//!
//! Checks for Local Administrator Password Solution (LAPS) settings.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// LAPS command
pub struct LAPSCommand;

impl LAPSCommand {
    /// Create a new LAPS command
    pub fn new() -> Self {
        Self
    }
}

impl Command for LAPSCommand {
    fn name(&self) -> &'static str {
        "LAPS"
    }

    fn description(&self) -> &'static str {
        "Local Administrator Password Solution (LAPS) settings, if installed"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let laps_path = "SOFTWARE\\Policies\\Microsoft Services\\AdmPwd";

        // Check if LAPS is installed and configured
        let admin_account_name = runtime
            .registry_get_string(RegistryHive::HKLM, laps_path, "AdminAccountName")?;

        let password_complexity = runtime
            .registry_get_dword(RegistryHive::HKLM, laps_path, "PasswordComplexity")?;

        let password_length = runtime
            .registry_get_dword(RegistryHive::HKLM, laps_path, "PasswordLength")?;

        let password_age_days = runtime
            .registry_get_dword(RegistryHive::HKLM, laps_path, "PasswordAgeDays")?;

        let adm_pwd_enabled = runtime
            .registry_get_dword(RegistryHive::HKLM, laps_path, "AdmPwdEnabled")?;

        // If no values found, LAPS is likely not installed
        if admin_account_name.is_none()
            && password_complexity.is_none()
            && password_length.is_none()
            && adm_pwd_enabled.is_none()
        {
            runtime.write_host("  [*] LAPS does not appear to be installed");
            return Ok(Vec::new());
        }

        let dto = LAPSDTO {
            installed: true,
            admin_account_name: admin_account_name.unwrap_or_else(|| "Not configured".to_string()),
            password_complexity: password_complexity.unwrap_or(0),
            password_length: password_length.unwrap_or(0),
            password_age_days: password_age_days.unwrap_or(0),
            enabled: adm_pwd_enabled.unwrap_or(0) == 1,
        };

        Ok(vec![Box::new(dto)])
    }
}

/// LAPS DTO
#[derive(Debug, Serialize)]
pub struct LAPSDTO {
    installed: bool,
    admin_account_name: String,
    password_complexity: u32,
    password_length: u32,
    password_age_days: u32,
    enabled: bool,
}

impl CommandDTO for LAPSDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  LAPS Installed:       {}\n  Enabled:              {}\n  Admin Account:        {}\n  Password Complexity:  {}\n  Password Length:      {}\n  Password Age (Days):  {}\n",
            self.installed,
            self.enabled,
            self.admin_account_name,
            self.password_complexity,
            self.password_length,
            self.password_age_days
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = LAPSCommand::new();
        assert_eq!(cmd.name(), "LAPS");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
