//! Windows Defender command
//!
//! Checks Windows Defender settings via registry.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Windows Defender command
pub struct WindowsDefenderCommand;

impl WindowsDefenderCommand {
    /// Create a new WindowsDefender command
    pub fn new() -> Self {
        Self
    }
}

impl Command for WindowsDefenderCommand {
    fn name(&self) -> &'static str {
        "WindowsDefender"
    }

    fn description(&self) -> &'static str {
        "Windows Defender settings (via Registry)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let base_path = "SOFTWARE\\Microsoft\\Windows Defender";

        // Check if Windows Defender is installed
        let disable_anti_spyware = runtime
            .registry_get_dword(RegistryHive::HKLM, base_path, "DisableAntiSpyware")?
            .unwrap_or(0);

        let disable_anti_virus = runtime
            .registry_get_dword(RegistryHive::HKLM, base_path, "DisableAntiVirus")?
            .unwrap_or(0);

        // Real-time protection settings
        let realtime_path = "SOFTWARE\\Microsoft\\Windows Defender\\Real-Time Protection";
        let disable_realtime_monitoring = runtime
            .registry_get_dword(RegistryHive::HKLM, realtime_path, "DisableRealtimeMonitoring")?
            .unwrap_or(0);

        let disable_behavior_monitoring = runtime
            .registry_get_dword(RegistryHive::HKLM, realtime_path, "DisableBehaviorMonitoring")?
            .unwrap_or(0);

        let disable_on_access_protection = runtime
            .registry_get_dword(RegistryHive::HKLM, realtime_path, "DisableOnAccessProtection")?
            .unwrap_or(0);

        let disable_scan_on_realtime_enable = runtime
            .registry_get_dword(
                RegistryHive::HKLM,
                realtime_path,
                "DisableScanOnRealtimeEnable",
            )?
            .unwrap_or(0);

        // Exclusions
        let exclusions_path = "SOFTWARE\\Microsoft\\Windows Defender\\Exclusions\\Paths";
        let exclusion_paths = runtime.registry_get_subkeys(RegistryHive::HKLM, exclusions_path)?;

        let dto = WindowsDefenderDTO {
            anti_spyware_enabled: disable_anti_spyware == 0,
            anti_virus_enabled: disable_anti_virus == 0,
            realtime_monitoring_enabled: disable_realtime_monitoring == 0,
            behavior_monitoring_enabled: disable_behavior_monitoring == 0,
            on_access_protection_enabled: disable_on_access_protection == 0,
            scan_on_realtime_enable_enabled: disable_scan_on_realtime_enable == 0,
            exclusion_count: exclusion_paths.len(),
        };

        Ok(vec![Box::new(dto)])
    }
}

/// Windows Defender settings DTO
#[derive(Debug, Serialize)]
pub struct WindowsDefenderDTO {
    anti_spyware_enabled: bool,
    anti_virus_enabled: bool,
    realtime_monitoring_enabled: bool,
    behavior_monitoring_enabled: bool,
    on_access_protection_enabled: bool,
    scan_on_realtime_enable_enabled: bool,
    exclusion_count: usize,
}

impl CommandDTO for WindowsDefenderDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Anti-Spyware:            {}\n  Anti-Virus:              {}\n  Real-time Monitoring:    {}\n  Behavior Monitoring:     {}\n  On-Access Protection:    {}\n  Scan on Enable:          {}\n  Path Exclusions:         {}\n",
            self.anti_spyware_enabled,
            self.anti_virus_enabled,
            self.realtime_monitoring_enabled,
            self.behavior_monitoring_enabled,
            self.on_access_protection_enabled,
            self.scan_on_realtime_enable_enabled,
            self.exclusion_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = WindowsDefenderCommand::new();
        assert_eq!(cmd.name(), "WindowsDefender");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
