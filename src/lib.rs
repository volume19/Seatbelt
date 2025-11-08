//! Seatbelt - Windows Security Enumeration Tool (Rust Port)
//!
//! This is a Rust port of the C# Seatbelt tool for Windows host enumeration
//! in authorized security assessments, penetration testing, and defensive operations.
//!
//! # Safety and Authorization
//!
//! This tool is intended for:
//! - Authorized penetration testing engagements
//! - Defensive security operations
//! - CTF challenges and security research
//! - Educational purposes
//!
//! Use only with proper authorization. Unauthorized access to computer systems is illegal.

#![warn(missing_docs)]

/// Error types and Result alias
pub mod error;

/// Command abstraction and base types
pub mod commands;
