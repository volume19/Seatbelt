//! Windows-specific enumeration commands

pub mod dns_cache;
pub mod network_shares;
pub mod environment_path;
pub mod secure_boot;
pub mod hotfixes;
pub mod local_users;
pub mod local_groups;
pub mod osinfo;
pub mod processes;
pub mod windows_defender;

pub use dns_cache::DnsCacheCommand;
pub use network_shares::NetworkSharesCommand;
pub use environment_path::EnvironmentPathCommand;
pub use secure_boot::SecureBootCommand;
pub use hotfixes::HotfixesCommand;
pub use local_users::LocalUsersCommand;
pub use local_groups::LocalGroupsCommand;
pub use osinfo::OSInfoCommand;
pub use processes::ProcessesCommand;
pub use windows_defender::WindowsDefenderCommand;
