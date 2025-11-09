//! Internet Settings command
//!
//! Checks Internet Explorer/Edge proxy and security settings.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Internet Settings command
pub struct InternetSettingsCommand;

impl InternetSettingsCommand {
    /// Create a new InternetSettings command
    pub fn new() -> Self {
        Self
    }
}

impl Command for InternetSettingsCommand {
    fn name(&self) -> &'static str {
        "InternetSettings"
    }

    fn description(&self) -> &'static str {
        "Internet settings including proxy configs and zones"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::USER | CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let ie_settings_path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

        // Proxy settings
        let proxy_enable = runtime
            .registry_get_dword(RegistryHive::HKCU, ie_settings_path, "ProxyEnable")?
            .unwrap_or(0);

        let proxy_server = runtime
            .registry_get_string(RegistryHive::HKCU, ie_settings_path, "ProxyServer")?
            .unwrap_or_default();

        let proxy_override = runtime
            .registry_get_string(RegistryHive::HKCU, ie_settings_path, "ProxyOverride")?
            .unwrap_or_default();

        let auto_config_url = runtime
            .registry_get_string(RegistryHive::HKCU, ie_settings_path, "AutoConfigURL")?
            .unwrap_or_default();

        // Zone settings - check Protected Mode for each zone
        let mut zones = Vec::new();
        let zone_names = vec![
            (0, "My Computer"),
            (1, "Local Intranet"),
            (2, "Trusted Sites"),
            (3, "Internet"),
            (4, "Restricted Sites"),
        ];

        for (zone_id, zone_name) in zone_names {
            let zone_path = format!(
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Zones\\{}",
                zone_id
            );

            if let Ok(Some(protected_mode)) =
                runtime.registry_get_dword(RegistryHive::HKCU, &zone_path, "2500")
            {
                zones.push((
                    zone_name.to_string(),
                    if protected_mode == 0 {
                        "Enabled".to_string()
                    } else {
                        "Disabled".to_string()
                    },
                ));
            }
        }

        let dto = InternetSettingsDTO {
            proxy_enabled: proxy_enable == 1,
            proxy_server,
            proxy_override,
            auto_config_url,
            zones,
        };

        Ok(vec![Box::new(dto)])
    }
}

/// Internet Settings DTO
#[derive(Debug, Serialize)]
pub struct InternetSettingsDTO {
    proxy_enabled: bool,
    proxy_server: String,
    proxy_override: String,
    auto_config_url: String,
    zones: Vec<(String, String)>,
}

impl CommandDTO for InternetSettingsDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        let mut output = format!(
            "  Proxy Enabled:    {}\n  Proxy Server:     {}\n  Proxy Override:   {}\n  Auto Config URL:  {}\n",
            self.proxy_enabled, self.proxy_server, self.proxy_override, self.auto_config_url
        );

        if !self.zones.is_empty() {
            output.push_str("\n  Protected Mode Settings:\n");
            for (zone, status) in &self.zones {
                output.push_str(&format!("    {}: {}\n", zone, status));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = InternetSettingsCommand::new();
        assert_eq!(cmd.name(), "InternetSettings");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
