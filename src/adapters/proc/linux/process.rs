use std::fs;

pub fn get_process_name(pid: u32) -> Option<String> {
    let comm_path = format!("/proc/{}/comm", pid);
    if let Ok(content) = fs::read_to_string(&comm_path) {
        return clean_process_name(&content);
    }
    None
}

fn clean_process_name(content: &str) -> Option<String> {
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn get_process_exe(pid: u32) -> Option<String> {
    let exe_path = format!("/proc/{}/exe", pid);
    fs::read_link(exe_path)
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_process_name() {
        assert_eq!(clean_process_name("systemd\n"), Some("systemd".to_string()));
        assert_eq!(clean_process_name("bash"), Some("bash".to_string()));
        assert_eq!(clean_process_name(""), None);
    }
}
