//! AppLocker command
//!
//! Checks AppLocker configuration and effective policies.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// AppLocker command
pub struct AppLockerCommand;

impl AppLockerCommand {
    /// Create a new AppLocker command
    pub fn new() -> Self {
        Self
    }
}

impl Command for AppLockerCommand {
    fn name(&self) -> &'static str {
        "AppLocker"
    }

    fn description(&self) -> &'static str {
        "AppLocker settings, if installed"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let applocker_path = "SOFTWARE\\Policies\\Microsoft\\Windows\\SrpV2";

        // Check if AppLocker is configured
        let rule_collections = vec!["Exe", "Dll", "Msi", "Script", "Appx"];
        let mut configured_rules = Vec::new();

        for collection in &rule_collections {
            let collection_path = format!("{}\\{}", applocker_path, collection);

            match runtime.registry_get_subkeys(RegistryHive::HKLM, &collection_path) {
                Ok(rules) if !rules.is_empty() => {
                    configured_rules.push((collection.to_string(), rules.len()));
                }
                _ => {}
            }
        }

        if configured_rules.is_empty() {
            runtime.write_host("  [*] AppLocker does not appear to be configured");
            return Ok(Vec::new());
        }

        // Check enforcement mode
        let enforcement_path = "SOFTWARE\\Policies\\Microsoft\\Windows\\SrpV2\\Exe";
        let enforcement_mode = runtime
            .registry_get_string(RegistryHive::HKLM, enforcement_path, "EnforcementMode")?;

        let dto = AppLockerDTO {
            configured: true,
            rule_collections: configured_rules,
            enforcement_mode: enforcement_mode.unwrap_or_else(|| "Unknown".to_string()),
        };

        Ok(vec![Box::new(dto)])
    }
}

/// AppLocker DTO
#[derive(Debug, Serialize)]
pub struct AppLockerDTO {
    configured: bool,
    rule_collections: Vec<(String, usize)>,
    enforcement_mode: String,
}

impl CommandDTO for AppLockerDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        let mut output = format!(
            "  AppLocker Configured: {}\n  Enforcement Mode:     {}\n\n  Rule Collections:\n",
            self.configured, self.enforcement_mode
        );

        for (collection, count) in &self.rule_collections {
            output.push_str(&format!("    {}: {} rules\n", collection, count));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = AppLockerCommand::new();
        assert_eq!(cmd.name(), "AppLocker");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
