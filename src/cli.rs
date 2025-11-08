//! Command-line interface argument parsing
//!
//! This module uses clap to parse command-line arguments and produce
//! a structured configuration for the Seatbelt runtime.

use clap::Parser;

/// Seatbelt - Windows Security Enumeration Tool
#[derive(Parser, Debug)]
#[command(name = "seatbelt")]
#[command(version = "1.2.2")]
#[command(about = "Windows host enumeration for authorized security assessments", long_about = None)]
pub struct CliArgs {
    /// Commands to execute (e.g., DNSCache, Processes)
    #[arg(value_name = "COMMAND")]
    pub commands: Vec<String>,

    /// Command group to execute (all, user, system, slack, chromium, remote, misc)
    #[arg(short = 'g', long = "group", value_name = "GROUP")]
    pub group: Option<String>,

    /// Output file path (.txt or .json)
    #[arg(short = 'o', long = "output-file", value_name = "FILE")]
    pub output_file: Option<String>,

    /// Quiet mode (suppress logo and verbose output)
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Return full results (disable filtering)
    #[arg(long = "full")]
    pub full: bool,

    /// Randomize command execution order
    #[arg(long = "randomize-order")]
    pub randomize_order: bool,

    /// Delay between commands in milliseconds
    #[arg(long = "delay-commands", value_name = "MS")]
    pub delay_commands: Option<u64>,

    /// Remote computer name for remote enumeration
    #[arg(long = "computer-name", value_name = "COMPUTER")]
    pub computer_name: Option<String>,

    /// Username for remote authentication (requires --password)
    #[arg(long = "username", value_name = "USER", requires = "password")]
    pub username: Option<String>,

    /// Password for remote authentication (requires --username)
    #[arg(long = "password", value_name = "PASS", requires = "username")]
    pub password: Option<String>,
}

impl CliArgs {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

/// Configuration derived from CLI arguments
#[derive(Debug, Clone)]
pub struct SeatbeltConfig {
    /// Individual commands to execute
    pub commands: Vec<String>,

    /// Command groups to execute
    pub command_groups: Vec<String>,

    /// Output file path (None = stdout)
    pub output_file: Option<String>,

    /// Filter results (true = filtered, false = full output)
    pub filter_results: bool,

    /// Quiet mode
    pub quiet_mode: bool,

    /// Randomize command execution order
    pub randomize_order: bool,

    /// Delay between commands in milliseconds
    pub delay_ms: Option<u64>,

    /// Remote computer name
    pub computer_name: Option<String>,

    /// Username for remote auth
    pub username: Option<String>,

    /// Password for remote auth
    pub password: Option<String>,
}

impl From<CliArgs> for SeatbeltConfig {
    fn from(args: CliArgs) -> Self {
        // Separate commands from groups
        // Commands prefixed with '-' are exclusions (handled by Runtime)
        let commands = args.commands;
        let command_groups = if let Some(group) = args.group {
            group.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            Vec::new()
        };

        Self {
            commands,
            command_groups,
            output_file: args.output_file,
            filter_results: !args.full,
            quiet_mode: args.quiet,
            randomize_order: args.randomize_order,
            delay_ms: args.delay_commands,
            computer_name: args.computer_name,
            username: args.username,
            password: args.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_args() {
        let args = CliArgs {
            commands: vec!["DNSCache".to_string(), "Processes".to_string()],
            group: Some("system".to_string()),
            output_file: Some("output.json".to_string()),
            quiet: true,
            full: false,
            randomize_order: false,
            delay_commands: Some(1000),
            computer_name: None,
            username: None,
            password: None,
        };

        let config = SeatbeltConfig::from(args);
        assert_eq!(config.commands.len(), 2);
        assert_eq!(config.command_groups, vec!["system"]);
        assert_eq!(config.output_file, Some("output.json".to_string()));
        assert!(config.quiet_mode);
        assert!(config.filter_results);
        assert_eq!(config.delay_ms, Some(1000));
    }

    #[test]
    fn test_multiple_groups() {
        let args = CliArgs {
            commands: vec![],
            group: Some("user,system,misc".to_string()),
            output_file: None,
            quiet: false,
            full: true,
            randomize_order: false,
            delay_commands: None,
            computer_name: None,
            username: None,
            password: None,
        };

        let config = SeatbeltConfig::from(args);
        assert_eq!(config.command_groups, vec!["user", "system", "misc"]);
        assert!(!config.filter_results); // full=true means no filtering
    }
}
