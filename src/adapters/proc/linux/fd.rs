use std::fs;

pub fn get_open_fds(pid: u32) -> Vec<u64> {
    let mut fds = Vec::new();
    let fd_dir = format!("/proc/{}/fd", pid);

    if let Ok(entries) = fs::read_dir(&fd_dir) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if let Ok(fd) = name.parse::<u64>() {
                    fds.push(fd);
                }
            }
        }
    }
    fds
}

pub fn count_open_files(pid: u32) -> usize {
    get_open_fds(pid).len()
}

#[allow(dead_code)]
fn parse_fd_name(name: &str) -> Option<u64> {
    name.parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fd_name() {
        assert_eq!(parse_fd_name("0"), Some(0));
        assert_eq!(parse_fd_name("100"), Some(100));
        assert_eq!(parse_fd_name("invalid"), None);
    }
}
