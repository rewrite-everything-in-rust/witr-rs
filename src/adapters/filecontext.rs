use crate::core::models::FileContext;

#[cfg(target_os = "linux")]
pub fn get_file_context(pid: u32) -> Option<FileContext> {
    use std::fs;
    
    let open_files = super::fd::count_open_files(pid);
    
    let limit_path = format!("/proc/{}/limits", pid);
    let mut file_limit = 0;
    
    if let Ok(content) = fs::read_to_string(&limit_path) {
        for line in content.lines() {
            if line.contains("Max open files") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(limit) = parts[3].parse::<u32>() {
                        file_limit = limit;
                    }
                }
                break;
            }
        }
    }
    
    let locked_files = get_locked_files_linux(pid);
    
    Some(FileContext {
        open_files,
        file_limit,
        locked_files,
        watched_dirs: Vec::new(),
    })
}

#[cfg(target_os = "linux")]
fn get_locked_files_linux(pid: u32) -> Vec<String> {
    use std::fs;
    
    let mut locked = Vec::new();
    
    if let Ok(content) = fs::read_to_string("/proc/locks") {
        let pid_str = pid.to_string();
        for line in content.lines() {
            if line.contains(&pid_str) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(path) = parts.last() {
                    locked.push(path.to_string());
                }
            }
        }
    }
    
    locked
}

#[cfg(target_os = "macos")]
pub fn get_file_context(pid: u32) -> Option<FileContext> {
    use std::process::Command;
    
    let mut open_files = 0;
    let mut locked_files = Vec::new();
    
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-F", "n"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.starts_with("n") {
                open_files += 1;
                if line.contains("LOCK") {
                    locked_files.push(line[1..].to_string());
                }
            }
        }
    }
    
    Some(FileContext {
        open_files,
        file_limit: 0,
        locked_files,
        watched_dirs: Vec::new(),
    })
}

#[cfg(target_os = "windows")]
pub fn get_file_context(_pid: u32) -> Option<FileContext> {
    None
}
