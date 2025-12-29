use std::process::Command;
use std::collections::HashMap;

pub fn get_socket_state(pid: u32) -> HashMap<u64, String> {
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-nP", "-iTCP"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_socket_states(&output_str);
    }
    
    HashMap::new()
}

fn parse_socket_states(output_str: &str) -> HashMap<u64, String> {
    let mut states = HashMap::new();
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let fd_str = parts[3].trim_end_matches(char::is_alphabetic);
            if let Ok(fd) = fd_str.parse::<u64>() {
                if let Some(state) = parts.get(9) {
                    states.insert(fd, state.to_string());
                }
            }
        }
    }
    states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_socket_states() {
        let output = "COMMAND   PID USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
main      123 user   10u  IPv4 0x...      0t0  TCP 127.0.0.1:8080 (LISTEN)
main      123 user   20u  IPv4 0x...      0t0  TCP 127.0.0.1:5432 (ESTABLISHED)
";
        let states = parse_socket_states(output);
        assert_eq!(states.len(), 2);
        assert_eq!(states.get(&10).map(|s| s.as_str()), Some("(LISTEN)"));
        assert_eq!(states.get(&20).map(|s| s.as_str()), Some("(ESTABLISHED)"));
    }
}
