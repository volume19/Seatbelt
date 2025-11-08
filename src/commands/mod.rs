//! Command abstraction and base types
//!
//! This module defines the core Command trait that all enumeration commands must implement.
//! It corresponds to the C# CommandBase abstract class.

pub mod dto;
pub mod groups;

use crate::error::Result;
pub use dto::CommandDTO;
pub use groups::CommandGroup;

/// Base trait for all Seatbelt commands
///
/// This trait is equivalent to the C# CommandBase abstract class.
/// Each command must implement this trait to be discoverable and executable.
///
/// # Example
///
/// ```ignore
/// struct MyCommand;
///
/// impl Command for MyCommand {
///     fn name(&self) -> &'static str { "MyCommand" }
///     fn description(&self) -> &'static str { "Does something useful" }
///     fn group(&self) -> CommandGroup { CommandGroup::SYSTEM }
///     fn supports_remote(&self) -> bool { true }
///
///     fn execute(&self, runtime: &Runtime, args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
///         // Implementation here
///         Ok(vec![])
///     }
/// }
/// ```
pub trait Command: Send + Sync {
    /// The command name (used for invocation)
    fn name(&self) -> &'static str;

    /// Human-readable description
    fn description(&self) -> &'static str;

    /// Command group(s) this command belongs to
    fn group(&self) -> CommandGroup;

    /// Whether this command supports remote execution
    fn supports_remote(&self) -> bool;

    /// Command version (default: "1.0")
    fn version(&self) -> &'static str {
        "1.0"
    }

    /// Execute the command
    ///
    /// # Arguments
    /// * `runtime` - The runtime context providing access to system resources
    /// * `args` - Command-specific arguments
    ///
    /// # Returns
    /// A vector of DTOs representing the command output
    fn execute(&self, runtime: &crate::runtime::Runtime, args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>>;
}
