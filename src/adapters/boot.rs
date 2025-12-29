#[cfg(target_os = "linux")]
pub fn get_boot_time() -> u64 {
    use std::fs;

    if let Ok(content) = fs::read_to_string("/proc/stat") {
        for line in content.lines() {
            if line.starts_with("btime ") {
                if let Some(time_str) = line.split_whitespace().nth(1) {
                    if let Ok(time) = time_str.parse::<u64>() {
                        return time;
                    }
                }
            }
        }
    }
    0
}

#[cfg(target_os = "macos")]
pub fn get_boot_time() -> u64 {
    use std::process::Command;

    if let Ok(output) = Command::new("sysctl")
        .args(["-n", "kern.boottime"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(start) = output_str.find("sec = ") {
            let rest = &output_str[start + 6..];
            if let Some(end) = rest.find(',') {
                if let Ok(time) = rest[..end].parse::<u64>() {
                    return time;
                }
            }
        }
    }
    0
}

#[cfg(target_os = "windows")]
pub fn get_boot_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .saturating_sub(60 * 60 * 24)
}
