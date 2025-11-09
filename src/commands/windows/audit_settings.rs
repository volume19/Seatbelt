//! Audit Settings command
//!
//! Checks Windows audit policy configuration.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Audit Settings command
pub struct AuditSettingsCommand;

impl AuditSettingsCommand {
    /// Create a new AuditSettings command
    pub fn new() -> Self {
        Self
    }
}

impl Command for AuditSettingsCommand {
    fn name(&self) -> &'static str {
        "AuditSettings"
    }

    fn description(&self) -> &'static str {
        "Classic and Advanced audit policy settings"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Check classic audit policy
        let audit_path = "SYSTEM\\CurrentControlSet\\Control\\Lsa";

        let audit_base_objects = runtime
            .registry_get_dword(RegistryHive::HKLM, audit_path, "AuditBaseObjects")?
            .unwrap_or(0);

        let audit_base_directories = runtime
            .registry_get_dword(RegistryHive::HKLM, audit_path, "AuditBaseDirectories")?
            .unwrap_or(0);

        let crash_on_audit_fail = runtime
            .registry_get_dword(RegistryHive::HKLM, audit_path, "CrashOnAuditFail")?
            .unwrap_or(0);

        dtos.push(Box::new(AuditSettingsDTO {
            audit_base_objects: audit_base_objects == 1,
            audit_base_directories: audit_base_directories == 1,
            crash_on_audit_fail: crash_on_audit_fail == 1,
        }));

        // Check if advanced audit policy is in use
        let adv_audit_path = "SYSTEM\\CurrentControlSet\\Control\\Lsa";
        let scenario_guid = runtime
            .registry_get_dword(RegistryHive::HKLM, adv_audit_path, "SCENoApplyLegacyAuditPolicy")?
            .unwrap_or(0);

        if scenario_guid == 1 {
            runtime.write_verbose("Advanced Audit Policy is enabled");
        } else {
            runtime.write_verbose("Using classic audit policy");
        }

        Ok(dtos)
    }
}

/// Audit Settings DTO
#[derive(Debug, Serialize)]
pub struct AuditSettingsDTO {
    audit_base_objects: bool,
    audit_base_directories: bool,
    crash_on_audit_fail: bool,
}

impl CommandDTO for AuditSettingsDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Audit Base Objects:     {}\n  Audit Base Directories: {}\n  Crash On Audit Fail:    {}\n",
            self.audit_base_objects, self.audit_base_directories, self.crash_on_audit_fail
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = AuditSettingsCommand::new();
        assert_eq!(cmd.name(), "AuditSettings");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
