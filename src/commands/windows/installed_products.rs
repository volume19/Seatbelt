//! Installed Products command
//!
//! Enumerates installed software via WMI (Win32_Product class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Installed Products command
pub struct InstalledProductsCommand;

impl InstalledProductsCommand {
    /// Create a new InstalledProducts command
    pub fn new() -> Self {
        Self
    }
}

impl Command for InstalledProductsCommand {
    fn name(&self) -> &'static str {
        "InstalledProducts"
    }

    fn description(&self) -> &'static str {
        "Installed products via WMI (via Win32_Product)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM | CommandGroup::MISC
    }

    fn supports_remote(&self) -> bool {
        true
    }

    fn execute(&self, runtime: &Runtime, _args: &[String]) -> Result<Vec<Box<dyn CommandDTO>>> {
        #[cfg(windows)]
        {
            execute_windows(runtime)
        }

        #[cfg(not(windows))]
        {
            runtime.write_warning("InstalledProducts command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_Product
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ProductWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Version")]
    version: Option<String>,
    #[serde(rename = "Vendor")]
    vendor: Option<String>,
    #[serde(rename = "InstallDate")]
    install_date: Option<String>,
}

/// Installed Product DTO
#[derive(Debug, Serialize)]
pub struct InstalledProductDTO {
    name: String,
    version: String,
    vendor: String,
    install_date: String,
}

impl CommandDTO for InstalledProductDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:         {}\n  Version:      {}\n  Vendor:       {}\n  Install Date: {}\n",
            self.name, self.version, self.vendor, self.install_date
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    runtime.write_verbose("Note: Win32_Product query can be slow and may trigger MSI repairs");

    let wmi = runtime.wmi_connection()?;
    let results: Vec<ProductWmi> =
        wmi.query("SELECT Name, Version, Vendor, InstallDate FROM Win32_Product")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(InstalledProductDTO {
                name: r.name.unwrap_or_default(),
                version: r.version.unwrap_or_default(),
                vendor: r.vendor.unwrap_or_default(),
                install_date: r.install_date.unwrap_or_default(),
            }) as Box<dyn CommandDTO>
        })
        .collect();

    Ok(dtos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        let cmd = InstalledProductsCommand::new();
        assert_eq!(cmd.name(), "InstalledProducts");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
        assert!(cmd.group().contains(CommandGroup::MISC));
    }
}
