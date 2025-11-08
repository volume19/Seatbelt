//! Windows Registry access utilities
//!
//! This module provides safe, cross-platform registry access.
//! On Windows, it uses the winreg crate. On other platforms, it provides
//! stub implementations that return empty results.

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

use crate::error::Result;
#[cfg(windows)]
use crate::error::SeatbeltError;
use std::collections::HashMap;

/// Registry hive enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryHive {
    /// HKEY_LOCAL_MACHINE
    HKLM,
    /// HKEY_CURRENT_USER
    HKCU,
    /// HKEY_CLASSES_ROOT
    HKCR,
    /// HKEY_USERS
    HKU,
    /// HKEY_CURRENT_CONFIG
    HKCC,
}

impl RegistryHive {
    #[cfg(windows)]
    fn to_regkey(&self) -> RegKey {
        match self {
            RegistryHive::HKLM => RegKey::predef(HKEY_LOCAL_MACHINE),
            RegistryHive::HKCU => RegKey::predef(HKEY_CURRENT_USER),
            RegistryHive::HKCR => RegKey::predef(HKEY_CLASSES_ROOT),
            RegistryHive::HKU => RegKey::predef(HKEY_USERS),
            RegistryHive::HKCC => RegKey::predef(HKEY_CURRENT_CONFIG),
        }
    }
}

/// Read a string value from the registry
///
/// # Arguments
/// * `hive` - Registry hive (HKLM, HKCU, etc.)
/// * `path` - Registry key path (e.g., "SOFTWARE\\Microsoft\\Windows\\CurrentVersion")
/// * `value` - Value name to read
///
/// # Returns
/// * `Ok(Some(String))` - Value exists and was read successfully
/// * `Ok(None)` - Key or value does not exist
/// * `Err(_)` - Access denied or other error
#[cfg(windows)]
pub fn get_string_value(hive: RegistryHive, path: &str, value: &str) -> Result<Option<String>> {
    let hkey = hive.to_regkey();

    match hkey.open_subkey(path) {
        Ok(key) => match key.get_value::<String, _>(value) {
            Ok(val) => Ok(Some(val)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SeatbeltError::RegistryError(format!(
                "Failed to read registry value: {}",
                e
            ))),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(SeatbeltError::AccessDenied(format!(
                "Access denied reading registry key: {}\\{}",
                path, value
            )))
        }
        Err(e) => Err(SeatbeltError::RegistryError(format!(
            "Failed to open registry key: {}",
            e
        ))),
    }
}

/// Non-Windows stub: always returns None
#[cfg(not(windows))]
pub fn get_string_value(_hive: RegistryHive, _path: &str, _value: &str) -> Result<Option<String>> {
    Ok(None)
}

/// Read a DWORD value from the registry
#[cfg(windows)]
pub fn get_dword_value(hive: RegistryHive, path: &str, value: &str) -> Result<Option<u32>> {
    let hkey = hive.to_regkey();

    match hkey.open_subkey(path) {
        Ok(key) => match key.get_value::<u32, _>(value) {
            Ok(val) => Ok(Some(val)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SeatbeltError::RegistryError(format!(
                "Failed to read registry value: {}",
                e
            ))),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(SeatbeltError::AccessDenied(format!(
                "Access denied reading registry key: {}\\{}",
                path, value
            )))
        }
        Err(e) => Err(SeatbeltError::RegistryError(format!(
            "Failed to open registry key: {}",
            e
        ))),
    }
}

/// Non-Windows stub: always returns None
#[cfg(not(windows))]
pub fn get_dword_value(_hive: RegistryHive, _path: &str, _value: &str) -> Result<Option<u32>> {
    Ok(None)
}

/// Read a binary value from the registry
#[cfg(windows)]
pub fn get_binary_value(hive: RegistryHive, path: &str, value: &str) -> Result<Option<Vec<u8>>> {
    let hkey = hive.to_regkey();

    match hkey.open_subkey(path) {
        Ok(key) => match key.get_raw_value(value) {
            Ok(val) => Ok(Some(val.bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SeatbeltError::RegistryError(format!(
                "Failed to read registry value: {}",
                e
            ))),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(SeatbeltError::AccessDenied(format!(
                "Access denied reading registry key: {}\\{}",
                path, value
            )))
        }
        Err(e) => Err(SeatbeltError::RegistryError(format!(
            "Failed to open registry key: {}",
            e
        ))),
    }
}

/// Non-Windows stub: always returns None
#[cfg(not(windows))]
pub fn get_binary_value(
    _hive: RegistryHive,
    _path: &str,
    _value: &str,
) -> Result<Option<Vec<u8>>> {
    Ok(None)
}

/// Enumerate subkey names under a registry key
#[cfg(windows)]
pub fn get_subkey_names(hive: RegistryHive, path: &str) -> Result<Vec<String>> {
    let hkey = hive.to_regkey();

    match hkey.open_subkey(path) {
        Ok(key) => {
            let subkeys: std::result::Result<Vec<String>, _> =
                key.enum_keys().collect();
            subkeys.map_err(|e| {
                SeatbeltError::RegistryError(format!("Failed to enumerate subkeys: {}", e))
            })
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(SeatbeltError::AccessDenied(format!(
                "Access denied reading registry key: {}",
                path
            )))
        }
        Err(e) => Err(SeatbeltError::RegistryError(format!(
            "Failed to open registry key: {}",
            e
        ))),
    }
}

/// Non-Windows stub: always returns empty vector
#[cfg(not(windows))]
pub fn get_subkey_names(_hive: RegistryHive, _path: &str) -> Result<Vec<String>> {
    Ok(Vec::new())
}

/// Get all values under a registry key as a HashMap
#[cfg(windows)]
pub fn get_values(hive: RegistryHive, path: &str) -> Result<HashMap<String, String>> {
    let hkey = hive.to_regkey();

    match hkey.open_subkey(path) {
        Ok(key) => {
            let mut values = HashMap::new();
            for (name, value) in key.enum_values().filter_map(|r| r.ok()) {
                if let Ok(val) = value.to_string() {
                    values.insert(name, val);
                }
            }
            Ok(values)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashMap::new()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(SeatbeltError::AccessDenied(format!(
                "Access denied reading registry key: {}",
                path
            )))
        }
        Err(e) => Err(SeatbeltError::RegistryError(format!(
            "Failed to open registry key: {}",
            e
        ))),
    }
}

/// Non-Windows stub: always returns empty HashMap
#[cfg(not(windows))]
pub fn get_values(_hive: RegistryHive, _path: &str) -> Result<HashMap<String, String>> {
    Ok(HashMap::new())
}

/// Get user SIDs from the registry
///
/// On Windows, enumerates HKEY_USERS to find user SIDs.
/// On other platforms, returns an empty vector.
#[cfg(windows)]
pub fn get_user_sids() -> Result<Vec<String>> {
    get_subkey_names(RegistryHive::HKU, "")
}

/// Non-Windows stub: always returns empty vector
#[cfg(not(windows))]
pub fn get_user_sids() -> Result<Vec<String>> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_hive_enum() {
        // Just ensure the enum exists and can be created
        let hive = RegistryHive::HKLM;
        assert_eq!(hive, RegistryHive::HKLM);
    }

    #[test]
    #[cfg(windows)]
    fn test_registry_read_string() {
        // Try to read a well-known registry value on Windows
        let result = get_string_value(
            RegistryHive::HKLM,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName",
        );

        // Should either succeed with Some value or return None (if key doesn't exist)
        // Should not error on a standard Windows system
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(not(windows))]
    fn test_registry_stub() {
        // On non-Windows, registry functions should return None/empty
        let string_result = get_string_value(RegistryHive::HKLM, "test", "test");
        assert_eq!(string_result.unwrap(), None);

        let dword_result = get_dword_value(RegistryHive::HKLM, "test", "test");
        assert_eq!(dword_result.unwrap(), None);

        let subkeys = get_subkey_names(RegistryHive::HKLM, "test").unwrap();
        assert!(subkeys.is_empty());
    }

    #[test]
    fn test_get_user_sids() {
        // Should not panic on any platform
        let sids = get_user_sids();
        assert!(sids.is_ok());
    }
}
