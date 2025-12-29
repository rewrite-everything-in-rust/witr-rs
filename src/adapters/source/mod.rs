pub mod darwin;
pub mod linux;

pub mod container;
pub mod cron;
pub mod detect;
pub mod git;
pub mod health;
pub mod network;
pub mod service;
pub mod shell;
pub mod supervisor;

// Re-export platform modules
#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "linux")]
pub use linux::*;

// Re-export shared modules
pub use container::*;
pub use cron::*;
pub use detect::*;
pub use git::*;
pub use health::*;
pub use network::*;
pub use service::*;
pub use shell::*;
pub use supervisor::*;
