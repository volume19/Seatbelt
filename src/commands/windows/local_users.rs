//! Local Users command
//!
//! Enumerates local user accounts via WMI (Win32_UserAccount class).

use crate::commands::{Command, CommandDTO, CommandGroup};
use crate::error::Result;
use crate::runtime::Runtime;
use serde::Serialize;

#[cfg(windows)]
use serde::Deserialize;

/// Local Users command
pub struct LocalUsersCommand;

impl LocalUsersCommand {
    /// Create a new LocalUsers command
    pub fn new() -> Self {
        Self
    }
}

impl Command for LocalUsersCommand {
    fn name(&self) -> &'static str {
        "LocalUsers"
    }

    fn description(&self) -> &'static str {
        "Local users, whether they're active/disabled, and pwd last set (via WMI)"
    }

    fn group(&self) -> CommandGroup {
        CommandGroup::USER | CommandGroup::SYSTEM
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
            runtime.write_warning("LocalUsers command is only supported on Windows");
            Ok(Vec::new())
        }
    }
}

/// WMI data structure for Win32_UserAccount
#[cfg(windows)]
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UserAccountWmi {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Domain")]
    domain: Option<String>,
    #[serde(rename = "SID")]
    sid: Option<String>,
    #[serde(rename = "Disabled")]
    disabled: Option<bool>,
    #[serde(rename = "LocalAccount")]
    local_account: Option<bool>,
    #[serde(rename = "PasswordChangeable")]
    password_changeable: Option<bool>,
    #[serde(rename = "PasswordExpires")]
    password_expires: Option<bool>,
    #[serde(rename = "PasswordRequired")]
    password_required: Option<bool>,
}

/// Local User DTO
#[derive(Debug, Serialize)]
pub struct LocalUserDTO {
    name: String,
    domain: String,
    sid: String,
    disabled: bool,
    local_account: bool,
    password_changeable: bool,
    password_expires: bool,
    password_required: bool,
}

impl CommandDTO for LocalUserDTO {
    fn command_version(&self) -> &str {
        "1.0"
    }

    fn format_text(&self) -> String {
        format!(
            "  Name:                 {}\n  Domain:               {}\n  SID:                  {}\n  Disabled:             {}\n  Local Account:        {}\n  Password Changeable:  {}\n  Password Expires:     {}\n  Password Required:    {}\n",
            self.name,
            self.domain,
            self.sid,
            self.disabled,
            self.local_account,
            self.password_changeable,
            self.password_expires,
            self.password_required
        )
    }
}

#[cfg(windows)]
fn execute_windows(runtime: &Runtime) -> Result<Vec<Box<dyn CommandDTO>>> {
    let wmi = runtime.wmi_connection()?;
    let results: Vec<UserAccountWmi> = wmi.query(
        "SELECT Name, Domain, SID, Disabled, LocalAccount, PasswordChangeable, PasswordExpires, PasswordRequired FROM Win32_UserAccount WHERE LocalAccount = True",
    )?;

    let dtos: Vec<Box<dyn CommandDTO>> = results
        .into_iter()
        .map(|r| {
            Box::new(LocalUserDTO {
                name: r.name.unwrap_or_default(),
                domain: r.domain.unwrap_or_default(),
                sid: r.sid.unwrap_or_default(),
                disabled: r.disabled.unwrap_or(false),
                local_account: r.local_account.unwrap_or(false),
                password_changeable: r.password_changeable.unwrap_or(false),
                password_expires: r.password_expires.unwrap_or(false),
                password_required: r.password_required.unwrap_or(false),
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
        let cmd = LocalUsersCommand::new();
        assert_eq!(cmd.name(), "LocalUsers");
        assert!(cmd.supports_remote());
        assert!(cmd.group().contains(CommandGroup::USER));
        assert!(cmd.group().contains(CommandGroup::SYSTEM));
    }
}
