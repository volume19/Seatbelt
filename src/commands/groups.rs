//! Command group definitions
//!
//! Commands are organized into groups for batch execution.
//! Groups use bitflags to allow commands to belong to multiple groups.

use bitflags::bitflags;

bitflags! {
    /// Command group flags
    ///
    /// These match the C# CommandGroup enum but use bitflags for
    /// efficient multi-group membership.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CommandGroup: u32 {
        /// All commands
        const ALL = 1 << 0;

        /// User-focused enumeration (history, credentials, etc.)
        const USER = 1 << 1;

        /// System-level enumeration (services, processes, etc.)
        const SYSTEM = 1 << 2;

        /// Slack-specific enumeration
        const SLACK = 1 << 3;

        /// Chromium browser enumeration
        const CHROMIUM = 1 << 4;

        /// Remote enumeration support
        const REMOTE = 1 << 5;

        /// Miscellaneous commands
        const MISC = 1 << 6;
    }
}

impl CommandGroup {
    /// Parse a group name from a string
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "all" => Some(Self::ALL),
            "user" => Some(Self::USER),
            "system" => Some(Self::SYSTEM),
            "slack" => Some(Self::SLACK),
            "chromium" => Some(Self::CHROMIUM),
            "remote" => Some(Self::REMOTE),
            "misc" => Some(Self::MISC),
            _ => None,
        }
    }

    /// Get the string name of this group
    pub fn name(&self) -> &'static str {
        match *self {
            Self::ALL => "All",
            Self::USER => "User",
            Self::SYSTEM => "System",
            Self::SLACK => "Slack",
            Self::CHROMIUM => "Chromium",
            Self::REMOTE => "Remote",
            Self::MISC => "Misc",
            _ => "Unknown",
        }
    }

    /// Get all defined groups
    pub fn all_groups() -> Vec<Self> {
        vec![Self::ALL, Self::USER, Self::SYSTEM, Self::SLACK, Self::CHROMIUM, Self::REMOTE, Self::MISC]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parse() {
        assert_eq!(CommandGroup::parse("all"), Some(CommandGroup::ALL));
        assert_eq!(CommandGroup::parse("USER"), Some(CommandGroup::USER));
        assert_eq!(CommandGroup::parse("System"), Some(CommandGroup::SYSTEM));
        assert_eq!(CommandGroup::parse("invalid"), None);
    }

    #[test]
    fn test_group_name() {
        assert_eq!(CommandGroup::ALL.name(), "All");
        assert_eq!(CommandGroup::SYSTEM.name(), "System");
    }

    #[test]
    fn test_bitflags_operations() {
        let combined = CommandGroup::USER | CommandGroup::SYSTEM;
        assert!(combined.contains(CommandGroup::USER));
        assert!(combined.contains(CommandGroup::SYSTEM));
        assert!(!combined.contains(CommandGroup::SLACK));
    }
}
