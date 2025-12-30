pub mod darwin;
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

pub mod container;
pub mod cron;
pub mod detect;
pub mod git;
pub mod health;
pub mod network;
pub mod service;
pub mod shell;
pub mod supervisor;

#[cfg(target_os = "macos")]
pub use darwin::*;

pub use container::*;
pub use cron::*;
pub use detect::*;
pub use git::*;
pub use health::*;
#[cfg(target_os = "linux")]
pub use linux::*;
pub use network::*;
pub use service::*;
pub use shell::*;
pub use supervisor::*;
