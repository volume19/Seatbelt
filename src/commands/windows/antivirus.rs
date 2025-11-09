//! AntiVirus command
//!
//! Enumerates installed antivirus products via WMI.

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// AntiVirus command
pub struct AntiVirusCommand;

impl AntiVirusCommand {
    /// Create a new AntiVirus command
    pub fn new() -> Self {
        Self
    }
}

impl Command for AntiVirusCommand {
    fn name(&self) -> &'static str {
        "AntiVirus"
    }

    fn description(&self) -> &'static str {
        "Installed antivirus products (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::SYSTEM
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
            runtime.write_warning("AntiVirus command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for AntiVirusProduct
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AntiVirusWmi {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "pathToSignedProductExe")]
    path_to_signed_product_exe: Option<String>,
    #[serde(rename = "pathToSignedReportingExe")]
    path_to_signed_reporting_exe: Option<String>,
}

/// AntiVirus DTO
#[derive(Debug, Serialize)]
pub struct AntiVirusDTO {
    display_name: String,
    product_exe: String,
    reporting_exe: String,
}

impl CommandDTO for AntiVirusDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Product:       {}\n  Product Exe:   {}\n  Reporting Exe: {}\n",
            self.display_name, self.product_exe, self.reporting_exe
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    // Try root\SecurityCenter2 first (Windows Vista and later)
    let wmi = runtime.wmi_connection_namespace("root\\SecurityCenter2")?;

    let results: Vec<AntiVirusWmi> = wmi.query(
        "SELECT displayName, pathToSignedProductExe, pathToSignedReportingExe FROM AntiVirusProduct",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(AntiVirusDTO {
                display_name: r.display_name.unwrap_or_default(),
                product_exe: r.path_to_signed_product_exe.unwrap_or_default(),
                reporting_exe: r.path_to_signed_reporting_exe.unwrap_or_default(),
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
        let cmd = AntiVirusCommand::new();
        assert_eq!(cmd.name(), "AntiVirus");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
