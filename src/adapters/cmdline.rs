#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
pub fn get_cmdline(pid: u32) -> Vec<String> {
    let path = format!("/proc/{}/cmdline", pid);
    if let Ok(bytes) = fs::read(&path) {
        return bytes
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect();
    }
    Vec::new()
}

#[cfg(target_os = "macos")]
pub fn get_cmdline(pid: u32) -> Vec<String> {
    use std::process::Command;

    if let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
    {
        let cmd_str = String::from_utf8_lossy(&output.stdout);
        return cmd_str.split_whitespace().map(|s| s.to_string()).collect();
    }
    Vec::new()
}

#[cfg(target_os = "windows")]
pub fn get_cmdline(_pid: u32) -> Vec<String> {
    Vec::new()
}
