//! Scheduled Tasks command
//!
//! Enumerates scheduled tasks via WMI (MSFT_ScheduledTask class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Scheduled Tasks command
pub struct ScheduledTasksCommand;

impl ScheduledTasksCommand {
    /// Create a new ScheduledTasks command
    pub fn new() -> Self {
        Self
    }
}

impl Command for ScheduledTasksCommand {
    fn name(&self) -> &'static str {
        "ScheduledTasks"
    }

    fn description(&self) -> &'static str {
        "Scheduled tasks (via WMI) that aren't authored by Microsoft"
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
            runtime.write_warning("ScheduledTasks command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for MSFT_ScheduledTask
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ScheduledTaskWmi {
    #[serde(rename = "TaskName")]
    task_name: Option<String>,
    #[serde(rename = "TaskPath")]
    task_path: Option<String>,
    #[serde(rename = "State")]
    state: Option<u32>,
    #[serde(rename = "Author")]
    author: Option<String>,
}

/// Scheduled Task DTO
#[derive(Debug, Serialize)]
pub struct ScheduledTaskDTO {
    task_name: String,
    task_path: String,
    state: String,
    author: String,
}

impl CommandDTO for ScheduledTaskDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Task Name: {}\n  Path:      {}\n  State:     {}\n  Author:    {}\n",
            self.task_name, self.task_path, self.state, self.author
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection_namespace("root\\Microsoft\\Windows\\TaskScheduler")?;
    let results: Vec<ScheduledTaskWmi> =
        wmi.query("SELECT TaskName, TaskPath, State, Author FROM MSFT_ScheduledTask")?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .filter(|r| {
            // Filter out Microsoft-authored tasks
            if let Some(ref author) = r.author {
                !author.contains("Microsoft") && !author.is_empty()
            } else {
                true
            }
        })
        .map(|r| {
            let state_str = match r.state {
                Some(0) => "Unknown",
                Some(1) => "Disabled",
                Some(2) => "Queued",
                Some(3) => "Ready",
                Some(4) => "Running",
                _ => "Unknown",
            };

            Box::new(ScheduledTaskDTO {
                task_name: r.task_name.unwrap_or_default(),
                task_path: r.task_path.unwrap_or_default(),
                state: state_str.to_string(),
                author: r.author.unwrap_or_default(),
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
        let cmd = ScheduledTasksCommand::new();
        assert_eq!(cmd.name(), "ScheduledTasks");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
