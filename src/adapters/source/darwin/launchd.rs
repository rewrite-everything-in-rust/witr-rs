use std::process::Command;

pub fn get_launchd_service(pid: u32) -> Option<String> {
    if let Ok(output) = Command::new("launchctl").args(["list"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_launchctl_list(&output_str, pid);
    }
    None
}

fn parse_launchctl_list(output: &str, pid: u32) -> Option<String> {
    let pid_str = pid.to_string();
    for line in output.lines() {
        // Format: PID Status Label
        // 1234 0 com.apple.something
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Some(p) = parts.first() {
                if *p == pid_str {
                    return parts.last().map(|s| s.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_launchctl_list() {
        let output = "PID\tStatus\tLabel
123\t0\tcom.example.service
456\t0\tcom.apple.finder
";
        assert_eq!(
            parse_launchctl_list(output, 123),
            Some("com.example.service".to_string())
        );
        assert_eq!(parse_launchctl_list(output, 999), None);
    }
}
