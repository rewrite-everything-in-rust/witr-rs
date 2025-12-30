#[cfg(target_os = "macos")]
pub mod darwin;
#[cfg(target_os = "linux")]
pub mod linux;

// Re-export platform-specific implementations
#[cfg(target_os = "macos")]
pub use darwin::net::{get_listening_sockets, get_sockets_for_pid, SocketInfo};
#[cfg(target_os = "macos")]
pub use darwin::socketstate::get_socket_state;

#[cfg(target_os = "linux")]
pub use linux::net::{get_listening_sockets, get_sockets_for_pid, SocketInfo};
#[cfg(target_os = "linux")]
pub use linux::socketstate::get_socket_state;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::net::{get_listening_sockets, get_sockets_for_pid};
#[cfg(target_os = "windows")]
pub use windows::socketstate::get_socket_state; // Use the robust implementation
                                                // Note: SocketInfo is now shared from core, we don't strictly need to re-export it from net specific if we use core everywhere
                                                // But for compatibility with system.rs usage of network::SocketInfo
#[cfg(target_os = "windows")]
pub use crate::core::models::SocketInfo;

// For non-mac/linux/windows platforms, provide stubs
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub mod stubs {
    use std::collections::HashMap;

    pub struct SocketInfo {
        pub port: u16,
        pub local_addr: String,
    }

    pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
        HashMap::new()
    }

    pub fn get_sockets_for_pid(_pid: u32) -> Vec<u64> {
        Vec::new()
    }

    pub fn get_socket_state(_pid: u32) -> HashMap<u64, String> {
        HashMap::new()
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub use stubs::*;
