//! Explicit Logon Events command
//!
//! Enumerates explicit logon events (Event ID 4648) from the Security log.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;

/// Explicit Logon Events command
pub struct ExplicitLogonEventsCommand;

impl ExplicitLogonEventsCommand {
    /// Create a new ExplicitLogonEvents command
    pub fn new() -> Self {
        Self
    }
}

impl Command for ExplicitLogonEventsCommand {
    fn name(&self) -> &'static str {
        "ExplicitLogonEvents"
    }

    fn description(&self) -> &'static str {
        "Explicit logon events (Event ID 4648) from the Security log"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM | CommandGroup::USER
    }

    fn supports_remote(&self) -> bool {
        false
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        // Note: Full implementation would use Windows Event Log API
        // This is a simplified version that notes the limitation

        runtime.write_host("  [*] Event log querying requires Windows Event Log API");
        runtime.write_host("  [*] This simplified implementation shows configuration only");
        runtime.write_verbose("Check Security log for Event ID 4648 manually");

        // Return empty for now - full implementation would query the event log
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = ExplicitLogonEventsCommand::new();
        assert_eq!(cmd.name(), "ExplicitLogonEvents");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
        assert!(cmd.group().contains(CommandGroup::USER));
    }
}
