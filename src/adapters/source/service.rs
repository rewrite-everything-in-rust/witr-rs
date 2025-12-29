#[cfg(target_os = "linux")]
use crate::adapters::source::linux::systemd;

#[cfg(target_os = "macos")]
use crate::adapters::source::darwin::launchd;

pub fn get_service_info(pid: u32) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        systemd::get_systemd_service(pid)
    }

    #[cfg(target_os = "macos")]
    {
        launchd::get_launchd_service(pid)
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

pub fn get_service_restart_count(service_name: &str) -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        systemd::get_restart_count(service_name)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = service_name;
        None
    }
}

pub fn get_service_file(service_name: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        systemd::get_fragment_path(service_name)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = service_name;
        None
    }
}
