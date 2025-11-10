//! WMI Event Filters command
//!
//! Enumerates WMI Event Filters (potential persistence mechanism).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// WMI Event Filters command
pub struct WMIEventFiltersCommand;

impl WMIEventFiltersCommand {
    /// Create a new WMIEventFilters command
    pub fn new() -> Self {
        Self
    }
}

impl Command for WMIEventFiltersCommand {
    fn name(&self) -> &'static str {
        "WMIEventFilters"
    }

    fn description(&self) -> &'static str {
        "WMI Event Filters (potential persistence)"
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
            runtime.write_warning("WMIEventFilters command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for __EventFilter
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct EventFilterWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Query")]
    query: Option<String>,
    #[serde(rename = "QueryLanguage")]
    query_language: Option<String>,
    #[serde(rename = "EventNamespace")]
    event_namespace: Option<String>,
}

/// WMI Event Filter DTO
#[derive(Debug, Serialize)]
pub struct WMIEventFilterDTO {
    name: String,
    query: String,
    query_language: String,
    event_namespace: String,
}

impl CommandDTO for WMIEventFilterDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:       {}\n  Namespace:  {}\n  Language:   {}\n  Query:      {}\n",
            self.name, self.event_namespace, self.query_language, self.query
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\subscription")?;
    let results: Vec<EventFilterWmi> =
        wmi.query("SELECT Name, Query, QueryLanguage, EventNamespace FROM __EventFilter")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(WMIEventFilterDTO {
                name: r.name.unwrap_or_default(),
                query: r.query.unwrap_or_default(),
                query_language: r.query_language.unwrap_or_default(),
                event_namespace: r.event_namespace.unwrap_or_default(),
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
        let cmd = WMIEventFiltersCommand::new();
        assert_eq!(cmd.name(), "WMIEventFilters");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
