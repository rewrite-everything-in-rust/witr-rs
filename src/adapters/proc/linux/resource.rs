use std::process::Command;

pub fn get_resource_context(pid: u32) -> Option<String> {
    if let Ok(output) = Command::new("systemd-inhibit")
        .args(["--list", "--no-pager"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if is_inhibiting(pid, &output_str) {
            return Some("sleep=true".to_string());
        }
    }
    None
}

fn is_inhibiting(pid: u32, output: &str) -> bool {
    // Output format usually:
    // WHO  UID  USER  PID  COMM  WHAT  WHY  MODE
    // ...  ...  ...   123  ...   ...   ...  ...
    let pid_str = pid.to_string();
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts.contains(&pid_str.as_str()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inhibiting() {
        let output = "     WHO       UID  USER  PID  COMM           WHAT                                                     WHY                                       MODE 
GNOME Shell  1000 user 1234 gnome-shell    handle-lid-switch                                        External monitor attached or configuration block     
Chrome       1000 user 5555 chrome         audio-playing                                            Playing Audio                             delay
";
        assert!(is_inhibiting(5555, output));
        assert!(is_inhibiting(1234, output));
        assert!(!is_inhibiting(9999, output));
    }

    #[test]
    fn test_get_resource_context() {
        let _ = get_resource_context(std::process::id());
    }
}
