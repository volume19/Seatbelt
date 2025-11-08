//! Utility modules
//!
//! This module provides helper functions and wrappers for common operations:
//! - WMI queries (Windows Management Instrumentation)
//! - Registry access (local and remote)
//! - Security/token utilities
//! - File operations

#[cfg(windows)]
pub mod wmi;

pub mod registry;
pub mod misc;

#[cfg(windows)]
pub use self::wmi::WmiConnection;

pub use registry::{RegistryHive, get_string_value, get_dword_value, get_binary_value, get_subkey_names, get_values, get_user_sids};
