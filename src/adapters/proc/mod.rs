#[cfg(target_os = "macos")]
pub mod darwin;
#[cfg(target_os = "linux")]
pub mod linux;

// Re-export platform-specific implementations
#[cfg(target_os = "macos")]
pub use crate::core::models::SocketInfo;
#[cfg(target_os = "macos")]
pub use darwin::net::{get_listening_sockets, get_sockets_for_pid};
#[cfg(target_os = "macos")]
pub use darwin::socketstate::get_socket_state;

#[cfg(target_os = "linux")]
pub use crate::core::models::SocketInfo;
#[cfg(target_os = "linux")]
pub use linux::net::{get_listening_sockets, get_sockets_for_pid};
#[cfg(target_os = "linux")]
pub use linux::socketstate::get_socket_state;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use crate::core::models::SocketInfo;
#[cfg(target_os = "windows")]
pub use windows::net::{get_listening_sockets, get_sockets_for_pid};
#[cfg(target_os = "windows")]
pub use windows::socketstate::get_socket_state;

// For non-mac/linux/windows platforms, provide stubs
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub mod stubs {
    use crate::core::models::SocketInfo;
    use std::collections::HashMap;

    pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
        HashMap::new()
    }

    pub fn get_sockets_for_pid(_pid: u32) -> Vec<u64> {
        Vec::new()
    }

    pub fn get_socket_state(_pid: u32) -> HashMap<u64, SocketInfo> {
        HashMap::new()
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub use stubs::*;
