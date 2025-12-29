#[cfg(target_os = "linux")]
use crate::adapters::source::linux::systemd;

#[cfg(target_os = "macos")]
use crate::adapters::source::darwin::launchd;

pub fn get_service_info(pid: u32) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        return systemd::get_systemd_service(pid);
    }

    #[cfg(target_os = "macos")]
    {
        return launchd::get_launchd_service(pid);
    }

    #[cfg(target_os = "windows")]
    {
        // Simple Windows Stub for now (or implement similar to linux/mac split)
        let _ = pid;
        None
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = pid;
        None
    }
}
