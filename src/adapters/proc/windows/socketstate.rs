use crate::core::models::SocketInfo;
use std::collections::HashMap;
use std::process::Command;

pub fn get_socket_state(target_pid: u32) -> HashMap<u64, SocketInfo> {
    let mut states = HashMap::new();

    if let Ok(output) = Command::new("netstat").args(["-ano"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            }

            let proto = parts[0];
            let local_addr = parts[1];
            let remote_addr = parts[2];
            let state = parts[3];
            let pid_str = parts[4];

            if proto == "TCP" {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    if pid == target_pid {
                        if let Some(port_part) = local_addr.rsplit(':').next() {
                            if let Ok(port) = port_part.parse::<u16>() {
                                let key = ((pid as u64) << 16) | (port as u64);

                                let info = SocketInfo::new(
                                    port,
                                    state.to_string(),
                                    local_addr.to_string(),
                                    remote_addr.to_string(),
                                );

                                states.insert(key, info);
                            }
                        }
                    }
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
    fn test_parse_socket_state_logic() {
        let info = SocketInfo::new(
            80,
            "TIME_WAIT".to_string(),
            "127.0.0.1:80".to_string(),
            "1.2.3.4:5678".to_string(),
        );

        assert_eq!(
            info.explanation,
            "Connection closed, waiting for delayed packets"
        );
        assert!(info.workaround.contains("SO_REUSEADDR"));
    }
}
