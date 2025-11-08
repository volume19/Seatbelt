//! Runtime execution engine
//!
//! The Runtime orchestrates command execution, manages output sinks,
//! and provides access to system resources (WMI, registry, etc.).

use crate::commands::{Command, CommandGroup};
use crate::error::Result;
use crate::output::OutputSink;
#[cfg(windows)]
use crate::util::WmiConnection;
use crate::util::{registry, RegistryHive};
use std::collections::HashMap;

/// Remote connection credentials
#[derive(Debug, Clone)]
pub struct RemoteConfig {
    /// Remote computer name
    pub computer_name: String,
    /// Optional username for authentication
    pub username: Option<String>,
    /// Optional password for authentication
    pub password: Option<String>,
}

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Filter command results
    pub filter_results: bool,
    /// Delay between commands in milliseconds
    pub delay_ms: Option<u64>,
    /// Randomize command execution order
    pub randomize_order: bool,
    /// Remote connection configuration
    pub remote: Option<RemoteConfig>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            filter_results: true,
            delay_ms: None,
            randomize_order: false,
            remote: None,
        }
    }
}

/// Runtime execution context
///
/// This struct provides commands with access to system resources and
/// orchestrates command execution.
pub struct Runtime {
    /// Registered commands
    commands: Vec<Box<dyn Command>>,
    /// Output sink
    output_sink: Box<dyn OutputSink>,
    /// Runtime configuration
    config: RuntimeConfig,
}

impl Runtime {
    /// Create a new runtime with the given output sink and configuration
    pub fn new(output_sink: Box<dyn OutputSink>, config: RuntimeConfig) -> Self {
        Self {
            commands: Vec::new(),
            output_sink,
            config,
        }
    }

    /// Register a command
    pub fn register_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// Register multiple commands
    pub fn register_commands(&mut self, commands: Vec<Box<dyn Command>>) {
        self.commands.extend(commands);
    }

    /// Get all registered commands
    pub fn commands(&self) -> &[Box<dyn Command>] {
        &self.commands
    }

    /// Check if running in remote mode
    pub fn is_remote(&self) -> bool {
        self.config.remote.is_some()
    }

    /// Get the filter results setting
    pub fn filter_results(&self) -> bool {
        self.config.filter_results
    }

    /// Execute a specific command by name
    pub fn execute_command(&mut self, command_name: &str, args: &[String]) -> Result<()> {
        // Find the command
        let command = self
            .commands
            .iter()
            .find(|c| c.name().eq_ignore_ascii_case(command_name))
            .ok_or_else(|| format!("Command not found: {}", command_name))?;

        // Check if command supports remote execution
        if self.is_remote() && !command.supports_remote() {
            self.output_sink.write_warning(&format!(
                "Command '{}' does not support remote execution",
                command_name
            ));
            return Ok(());
        }

        // Write command header
        self.output_sink
            .write_host(&format!("\n====== {} ======\n", command.name()));

        // Execute command
        match command.execute(self, args) {
            Ok(results) => {
                for dto in results {
                    self.output_sink.write_output(dto.as_ref())?;
                }
                Ok(())
            }
            Err(e) => {
                self.output_sink
                    .write_error(&format!("Error executing command '{}': {}", command_name, e));
                Err(e)
            }
        }
    }

    /// Execute commands by group
    pub fn execute_group(&mut self, group_name: &str) -> Result<()> {
        let group = CommandGroup::parse(group_name)
            .ok_or_else(|| format!("Invalid command group: {}", group_name))?;

        // Get all commands in this group
        let command_names: Vec<String> = self
            .commands
            .iter()
            .filter(|c| c.group().contains(group))
            .map(|c| c.name().to_string())
            .collect();

        self.output_sink.write_host(&format!(
            "\n[*] Executing group '{}' ({} commands)\n",
            group_name,
            command_names.len()
        ));

        for name in command_names {
            // Apply delay if configured
            if let Some(delay) = self.config.delay_ms {
                std::thread::sleep(std::time::Duration::from_millis(delay));
            }

            // Execute command, but continue on error
            if let Err(e) = self.execute_command(&name, &[]) {
                self.output_sink
                    .write_error(&format!("Failed to execute '{}': {}", name, e));
            }
        }

        Ok(())
    }

    /// Execute multiple commands by name
    pub fn execute_commands(&mut self, command_names: &[String]) -> Result<()> {
        for name in command_names {
            // Apply delay if configured
            if let Some(delay) = self.config.delay_ms {
                std::thread::sleep(std::time::Duration::from_millis(delay));
            }

            // Execute command, but continue on error
            if let Err(e) = self.execute_command(name, &[]) {
                self.output_sink
                    .write_error(&format!("Failed to execute '{}': {}", name, e));
            }
        }

        Ok(())
    }

    // ====== System Resource Access Methods ======

    /// Get a WMI connection
    #[cfg(windows)]
    pub fn wmi_connection(&self) -> Result<WmiConnection> {
        // TODO: Support remote WMI connections
        WmiConnection::new()
    }

    /// Get a WMI connection with a specific namespace
    #[cfg(windows)]
    pub fn wmi_connection_namespace(&self, namespace: &str) -> Result<WmiConnection> {
        // TODO: Support remote WMI connections
        WmiConnection::with_namespace(namespace)
    }

    /// Read a string value from the registry
    pub fn registry_get_string(
        &self,
        hive: RegistryHive,
        path: &str,
        value: &str,
    ) -> Result<Option<String>> {
        // TODO: Support remote registry access
        registry::get_string_value(hive, path, value)
    }

    /// Read a DWORD value from the registry
    pub fn registry_get_dword(
        &self,
        hive: RegistryHive,
        path: &str,
        value: &str,
    ) -> Result<Option<u32>> {
        // TODO: Support remote registry access
        registry::get_dword_value(hive, path, value)
    }

    /// Read a binary value from the registry
    pub fn registry_get_binary(
        &self,
        hive: RegistryHive,
        path: &str,
        value: &str,
    ) -> Result<Option<Vec<u8>>> {
        // TODO: Support remote registry access
        registry::get_binary_value(hive, path, value)
    }

    /// Enumerate registry subkeys
    pub fn registry_get_subkeys(&self, hive: RegistryHive, path: &str) -> Result<Vec<String>> {
        // TODO: Support remote registry access
        registry::get_subkey_names(hive, path)
    }

    /// Get all values under a registry key
    pub fn registry_get_values(
        &self,
        hive: RegistryHive,
        path: &str,
    ) -> Result<HashMap<String, String>> {
        // TODO: Support remote registry access
        registry::get_values(hive, path)
    }

    /// Get user SIDs from the registry
    pub fn get_user_sids(&self) -> Result<Vec<String>> {
        // TODO: Support remote registry access
        registry::get_user_sids()
    }

    /// Write a host message
    pub fn write_host(&mut self, message: &str) {
        self.output_sink.write_host(message);
    }

    /// Write an error message
    pub fn write_error(&mut self, message: &str) {
        self.output_sink.write_error(message);
    }

    /// Write a verbose message
    pub fn write_verbose(&mut self, message: &str) {
        self.output_sink.write_verbose(message);
    }

    /// Write a warning message
    pub fn write_warning(&mut self, message: &str) {
        self.output_sink.write_warning(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{CommandDTO, CommandGroup};
    use crate::error::Result;
    use crate::output::TextOutputSink;
    use crate::output::writer::TextWriter;

    // Mock command for testing
    struct MockCommand {
        name: &'static str,
    }

    impl Command for MockCommand {
        fn name(&self) -> &'static str {
            self.name
        }

        fn description(&self) -> &'static str {
            "Mock command"
        }

        fn group(&self) -> CommandGroup {
            CommandGroup::SYSTEM
        }

        fn supports_remote(&self) -> bool {
            true
        }

        fn execute(&self, _runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
            Ok(Vec::new())
        }
    }

    struct MockWriter {
        buffer: String,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                buffer: String::new(),
            }
        }
    }

    impl TextWriter for MockWriter {
        fn write(&mut self, text: &str) -> Result<()> {
            self.buffer.push_str(text);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_runtime_creation() {
        let writer = MockWriter::new();
        let sink = Box::new(TextOutputSink::new(writer, false));
        let runtime = Runtime::new(sink, RuntimeConfig::default());

        assert_eq!(runtime.commands().len(), 0);
        assert!(!runtime.is_remote());
    }

    #[test]
    fn test_register_commands() {
        let writer = MockWriter::new();
        let sink = Box::new(TextOutputSink::new(writer, false));
        let mut runtime = Runtime::new(sink, RuntimeConfig::default());

        runtime.register_command(Box::new(MockCommand { name: "Test1" }));
        runtime.register_command(Box::new(MockCommand { name: "Test2" }));

        assert_eq!(runtime.commands().len(), 2);
    }

    #[test]
    fn test_remote_config() {
        let writer = MockWriter::new();
        let sink = Box::new(TextOutputSink::new(writer, false));

        let config = RuntimeConfig {
            filter_results: false,
            delay_ms: Some(100),
            randomize_order: false,
            remote: Some(RemoteConfig {
                computer_name: "remote-pc".to_string(),
                username: Some("admin".to_string()),
                password: Some("pass".to_string()),
            }),
        };

        let runtime = Runtime::new(sink, config);
        assert!(runtime.is_remote());
    }
}
