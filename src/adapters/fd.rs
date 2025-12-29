#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
pub fn get_open_fds(pid: u32) -> Vec<u64> {
    let fd_dir = format!("/proc/{}/fd", pid);
    let mut inodes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&fd_dir) {
        for entry in entries.flatten() {
            if let Ok(link) = fs::read_link(entry.path()) {
                let link_str = link.to_string_lossy();
                if link_str.starts_with("socket:[") {
                    if let Some(inode_str) = link_str.strip_prefix("socket:[").and_then(|s| s.strip_suffix(']')) {
                        if let Ok(inode) = inode_str.parse::<u64>() {
                            inodes.push(inode);
                        }
                    }
                }
            }
        }
    }
    
    inodes
}

#[cfg(target_os = "macos")]
pub fn get_open_fds(pid: u32) -> Vec<u64> {
    use std::process::Command;
    
    let mut inodes = Vec::new();
    
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-nP", "-F", "n"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.starts_with("n") && line.contains(":") {
                if let Some(inode) = line.split(':').last() {
                    if let Ok(inode_num) = inode.parse::<u64>() {
                        inodes.push(inode_num);
                    }
                }
            }
        }
    }
    
    inodes
}

#[cfg(target_os = "windows")]
pub fn get_open_fds(pid: u32) -> Vec<u64> {
    vec![pid as u64]
}

#[cfg(target_os = "linux")]
pub fn count_open_files(pid: u32) -> u32 {
    let fd_dir = format!("/proc/{}/fd", pid);
    if let Ok(entries) = fs::read_dir(&fd_dir) {
        return entries.count() as u32;
    }
    0
}

#[cfg(not(target_os = "linux"))]
pub fn count_open_files(_pid: u32) -> u32 {
    0
}
