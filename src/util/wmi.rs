//! WMI (Windows Management Instrumentation) utilities
//!
//! This module provides a safe wrapper around the `wmi` crate for querying
//! system information via WMI, both locally and remotely.

use crate::error::{Result, SeatbeltError};
use serde::de::DeserializeOwned;
use wmi::{COMLibrary, WMIConnection as WmiConnectionInner};

/// WMI connection wrapper
///
/// Provides safe access to WMI queries with proper error handling.
pub struct WmiConnection {
    com_con: COMLibrary,
    wmi_con: WmiConnectionInner,
}

impl WmiConnection {
    /// Create a new local WMI connection
    ///
    /// # Example
    /// ```ignore
    /// let wmi = WmiConnection::new()?;
    /// let results: Vec<Process> = wmi.query("SELECT * FROM Win32_Process")?;
    /// ```
    pub fn new() -> Result<Self> {
        let com_con = COMLibrary::new().map_err(|e| {
            SeatbeltError::WmiError(format!("Failed to initialize COM library: {}", e))
        })?;

        let wmi_con = WMIConnection::new(com_con.clone()).map_err(|e| {
            SeatbeltError::WmiError(format!("Failed to create WMI connection: {}", e))
        })?;

        Ok(Self { com_con, wmi_con })
    }

    /// Create a new WMI connection with custom namespace
    ///
    /// # Arguments
    /// * `namespace` - WMI namespace (e.g., "root\\cimv2", "root\\standardcimv2")
    pub fn with_namespace(namespace: &str) -> Result<Self> {
        let com_con = COMLibrary::new().map_err(|e| {
            SeatbeltError::WmiError(format!("Failed to initialize COM library: {}", e))
        })?;

        let wmi_con = WMIConnection::with_namespace_path(namespace, com_con.clone()).map_err(
            |e| SeatbeltError::WmiError(format!("Failed to create WMI connection: {}", e)),
        )?;

        Ok(Self { com_con, wmi_con })
    }

    /// Execute a WMI query and deserialize results
    ///
    /// # Type Parameters
    /// * `T` - Type implementing `serde::Deserialize` with field names matching WMI properties
    ///
    /// # Arguments
    /// * `query` - WQL query string (e.g., "SELECT * FROM Win32_Process")
    ///
    /// # Returns
    /// Vector of deserialized results
    pub fn query<T: DeserializeOwned>(&self, query: &str) -> Result<Vec<T>> {
        self.wmi_con
            .raw_query(query)
            .map_err(|e| SeatbeltError::WmiError(format!("WMI query failed: {}", e)))
    }

    /// Execute a WMI query and return a specific number of results
    ///
    /// Useful for queries where you expect exactly one result.
    pub fn query_single<T: DeserializeOwned>(&self, query: &str) -> Result<Option<T>> {
        let mut results: Vec<T> = self.query(query)?;
        Ok(results.pop())
    }
}

/// Helper function to create a local WMI connection
pub fn connect() -> Result<WmiConnection> {
    WmiConnection::new()
}

/// Helper function to create a WMI connection with a specific namespace
pub fn connect_namespace(namespace: &str) -> Result<WmiConnection> {
    WmiConnection::with_namespace(namespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run on Windows with WMI available
    fn test_wmi_connection() {
        // This test requires Windows and WMI to be available
        if cfg!(windows) {
            let wmi = WmiConnection::new();
            assert!(wmi.is_ok());
        }
    }

    #[test]
    fn test_helper_functions() {
        // Just test that functions exist and can be called (they'll fail on non-Windows)
        let _ = connect();
        let _ = connect_namespace("root\\cimv2");
    }
}
