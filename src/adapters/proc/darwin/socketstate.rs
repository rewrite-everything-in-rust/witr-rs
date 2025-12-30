use crate::core::models::SocketInfo;
use std::collections::HashMap;
use std::process::Command;

pub fn get_socket_state(pid: u32) -> HashMap<u64, SocketInfo> {
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-nP", "-iTCP"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_socket_states(&output_str);
    }

    HashMap::new()
}

fn parse_socket_states(output_str: &str) -> HashMap<u64, SocketInfo> {
    let mut states = HashMap::new();
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 9 {
            let fd_str = parts[3].trim_end_matches(char::is_alphabetic);

            // Format: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
            // NAME field (last) format: 127.0.0.1:5432->127.0.0.1:5433 (ESTABLISHED)
            // Or: *:8080 (LISTEN)

            let name_part = parts[8];
            let state_part = if parts.len() > 9 {
                parts[9]
            } else {
                "(UNKNOWN)"
            };
            let state = state_part.trim_matches(&['(', ')'] as &[_]).to_string();

            if let Ok(fd) = fd_str.parse::<u64>() {
                // Parse Local Address and Port from NAME
                // Case 1: *:8080
                // Case 2: 127.0.0.1:5432
                // Case 3: 127.0.0.1:5432->...

                let addrs: Vec<&str> = name_part.split("->").collect();
                let local = addrs[0];
                let remote = if addrs.len() > 1 { addrs[1] } else { "" };

                let mut port = 0;
                if let Some(port_str) = local.rsplit(':').next() {
                    if let Ok(p) = port_str.parse::<u16>() {
                        port = p;
                    }
                }

                let info = SocketInfo::new(port, state, local.to_string(), remote.to_string());
                states.insert(fd, info);
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
main      123 user   20u  IPv4 0x...      0t0  TCP 127.0.0.1:5432->1.1.1.1:80 (ESTABLISHED)
";
        let states = parse_socket_states(output);
        assert_eq!(states.len(), 2);

        if let Some(info) = states.get(&10) {
            assert_eq!(info.state, "LISTEN");
            assert_eq!(info.port, 8080);
            assert_eq!(info.local_addr, "127.0.0.1:8080");
        }

        if let Some(info) = states.get(&20) {
            assert_eq!(info.state, "ESTABLISHED");
            assert_eq!(info.remote_addr, "1.1.1.1:80");
        }
    }
}
