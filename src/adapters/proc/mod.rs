pub mod darwin;
pub mod linux;

// Re-export platform-specific implementations
#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "linux")]
pub use linux::*;

// For non-mac/linux platforms, provide stubs
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
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
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub use stubs::*;
