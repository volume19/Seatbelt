//! Utility modules
//!
//! This module provides helper functions and wrappers for common operations:
//! - WMI queries (Windows Management Instrumentation)
//! - Registry access (local and remote)
//! - Security/token utilities
//! - File operations

#[cfg(windows)]
pub mod wmi;

pub mod misc;

#[cfg(windows)]
pub use self::wmi::WmiConnection;
