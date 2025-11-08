//! Error types for Seatbelt
//!
//! This module defines the comprehensive error handling strategy for the Seatbelt tool.
//! All fallible operations return `Result<T, SeatbeltError>`.

/// Main error type for Seatbelt operations
#[derive(thiserror::Error, Debug)]
pub enum SeatbeltError {
    /// IO errors (file operations, network, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// WMI query or connection errors
    #[error("WMI error: {0}")]
    WmiError(String),

    /// Windows Registry access errors
    #[error("Registry error: {0}")]
    RegistryError(String),

    /// Access denied / insufficient privileges
    #[error("Access denied: {0}")]
    AccessDenied(String),

    /// Windows API call failures
    #[error("Windows API error: {0}")]
    WindowsApiError(String),

    /// Command execution errors
    #[error("Command execution error: {0}")]
    CommandError(String),

    /// Parsing errors (JSON, XML, etc.)
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// General errors
    #[error("{0}")]
    Other(String),
}

/// Convenient Result type alias
pub type Result<T> = std::result::Result<T, SeatbeltError>;

// Conversion from serde_json::Error
impl From<serde_json::Error> for SeatbeltError {
    fn from(err: serde_json::Error) -> Self {
        SeatbeltError::SerializationError(err.to_string())
    }
}

// Conversion from string (for quick error construction)
impl From<String> for SeatbeltError {
    fn from(msg: String) -> Self {
        SeatbeltError::Other(msg)
    }
}

impl From<&str> for SeatbeltError {
    fn from(msg: &str) -> Self {
        SeatbeltError::Other(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_string() {
        let err = SeatbeltError::from("test error");
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn test_error_variants() {
        let io_err = SeatbeltError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        assert!(io_err.to_string().contains("file not found"));

        let wmi_err = SeatbeltError::WmiError("connection failed".to_string());
        assert_eq!(wmi_err.to_string(), "WMI error: connection failed");

        let reg_err = SeatbeltError::RegistryError("key not found".to_string());
        assert_eq!(reg_err.to_string(), "Registry error: key not found");
    }
}
