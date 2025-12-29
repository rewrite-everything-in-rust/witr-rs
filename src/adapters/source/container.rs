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

pub fn get_container_id(pid: u32) -> Option<String> {
    let cgroup_path = format!("/proc/{}/cgroup", pid);
    if let Ok(content) = fs::read_to_string(&cgroup_path) {
        return parse_cgroup_id(&content);
    }
    None
}

fn parse_cgroup_id(content: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(idx) = line.find("/docker/") {
            let id = &line[idx + 8..];
            return Some(id.trim().to_string());
        }
    }
    None
}

pub fn get_docker_restart_count(container_id: &str) -> Option<u32> {
    use std::process::Command;
    if let Ok(output) = Command::new("docker")
        .args(["inspect", "-f", "{{.RestartCount}}", container_id])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        return s.trim().parse().ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cgroup() {
        assert_eq!(
            parse_cgroup("1:name=systemd:/docker/12345"),
            Some("docker".to_string())
        );
        assert_eq!(
            parse_cgroup("1:name=systemd:/kubepods/123"),
            Some("kubernetes".to_string())
        );
        assert_eq!(parse_cgroup("1:name=systemd:/user.slice"), None);
    }

    #[test]
    fn test_parse_cgroup_id() {
        assert_eq!(
            parse_cgroup_id("1:name=systemd:/docker/12345abcdef"),
            Some("12345abcdef".to_string())
        );
        assert_eq!(
            parse_cgroup_id("11:pids:/docker/c50c18d45123"),
            Some("c50c18d45123".to_string())
        );
        assert_eq!(parse_cgroup_id("1:name=systemd:/user.slice"), None);
    }
}
