//! Data Transfer Object (DTO) base types for command outputs

use serde::Serialize;

/// Base trait for all command output DTOs
///
/// This trait must be implemented by all command output types.
/// It combines serialization (via erased-serde) with versioning and text formatting.
pub trait CommandDTO: erased_serde::Serialize + Send + Sync {
    /// Get the command version that produced this DTO
    fn command_version(&self) -> &str;

    /// Format this DTO as human-readable text
    ///
    /// Default implementation uses Debug formatting, but commands
    /// can provide custom formatting via explicit implementations.
    fn format_text(&self) -> String {
        format!("{:?}", self as *const Self)
    }
}

// Enable trait object serialization for CommandDTO
erased_serde::serialize_trait_object!(CommandDTO);

/// Base DTO for host/console messages
#[derive(Debug, Clone, Serialize)]
pub struct HostDTO {
    /// The message content
    pub message: String,

    #[serde(skip)]
    version: String,
}

impl HostDTO {
    /// Create a new host message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            version: "1.0".to_string(),
        }
    }
}

impl CommandDTO for HostDTO {
    fn command_version(&self) -> &str {
        &self.version
    }

    fn format_text(&self) -> String {
        self.message.clone()
    }
}

/// Macro to help implement CommandDTO for simple types
#[macro_export]
macro_rules! impl_command_dto {
    ($type:ty, $version:expr) => {
        impl CommandDTO for $type {
            fn command_version(&self) -> &str {
                $version
            }
        }
    };
}
