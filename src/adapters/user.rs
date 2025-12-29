#[cfg(target_os = "linux")]
pub fn get_username(uid: &str) -> Option<String> {
    use std::fs;
    
    if let Ok(content) = fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 && parts[2] == uid {
                return Some(parts[0].to_string());
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
pub fn get_username(uid: &str) -> Option<String> {
    use std::process::Command;
    
    if let Ok(output) = Command::new("id").args(["-un", uid]).output() {
        let username = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !username.is_empty() {
            return Some(username);
        }
    }
    None
}

#[cfg(target_os = "windows")]
pub fn get_username(_uid: &str) -> Option<String> {
    None
}
