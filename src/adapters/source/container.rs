use std::fs;

pub fn detect_container(pid: u32) -> Option<String> {
    let cgroup_path = format!("/proc/{}/cgroup", pid);
    if let Ok(content) = fs::read_to_string(&cgroup_path) {
        return parse_cgroup(&content);
    }
    None
}

fn parse_cgroup(content: &str) -> Option<String> {
    if content.contains("docker") {
        return Some("docker".to_string());
    } else if content.contains("containerd") {
        return Some("containerd".to_string());
    } else if content.contains("kubepods") {
        return Some("kubernetes".to_string());
    }
    None
}

pub fn is_container_process(pid: u32) -> bool {
    detect_container(pid).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cgroup() {
        assert_eq!(parse_cgroup("1:name=systemd:/docker/12345"), Some("docker".to_string()));
        assert_eq!(parse_cgroup("1:name=systemd:/kubepods/123"), Some("kubernetes".to_string()));
        assert_eq!(parse_cgroup("1:name=systemd:/user.slice"), None);
    }
}
