//! Certificates command
//!
//! Enumerates installed certificates from various stores.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use crate::util::RegistryHive;
use serde::Serialize;

/// Certificates command
pub struct CertificatesCommand;

impl CertificatesCommand {
    /// Create a new Certificates command
    pub fn new() -> Self {
        Self
    }
}

impl Command for CertificatesCommand {
    fn name(&self) -> &'static str {
        "Certificates"
    }

    fn description(&self) -> &'static str {
        "Interesting certificates from the user and machine certificate stores"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::USER | CommandGroup::MISC
    }

    fn supports_remote(&self) -> bool {
        false
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        // For a full implementation, we'd use Windows crypto APIs
        // This simplified version checks for certificate registry locations

        let mut dtos: Vec<Box<dyn CommandDTO>> = Vec::new();

        // Check common certificate store locations in registry
        let cert_stores = vec![
            (RegistryHive::HKLM, "SOFTWARE\\Microsoft\\SystemCertificates\\ROOT\\Certificates", "Machine Root CA"),
            (RegistryHive::HKLM, "SOFTWARE\\Microsoft\\SystemCertificates\\CA\\Certificates", "Machine Intermediate CA"),
            (RegistryHive::HKCU, "SOFTWARE\\Microsoft\\SystemCertificates\\My\\Certificates", "User Personal"),
        ];

        for (hive, path, store_name) in cert_stores {
            match runtime.registry_get_subkeys(hive, path) {
                Ok(thumbprints) => {
                    runtime.write_verbose(&format!("Found {} certificates in {}", thumbprints.len(), store_name));

                    dtos.push(Box::new(CertificateStoreDTO {
                        store_name: store_name.to_string(),
                        certificate_count: thumbprints.len(),
                    }));
                }
                Err(_) => {
                    // Store doesn't exist or can't be accessed
                }
            }
        }

        if dtos.is_empty() {
            runtime.write_host("  [*] Note: Full certificate enumeration requires Windows CryptoAPI");
            runtime.write_host("  [*] This simplified version shows certificate store locations");
        }

        Ok(dtos)
    }
}

/// Certificate Store DTO
#[derive(Debug, Serialize)]
pub struct CertificateStoreDTO {
    store_name: String,
    certificate_count: usize,
}

impl CommandDTO for CertificateStoreDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Store:            {}\n  Certificate Count: {}\n",
            self.store_name, self.certificate_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = CertificatesCommand::new();
        assert_eq!(cmd.name(), "Certificates");
        assert!(!cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::MISC));
    }
}
