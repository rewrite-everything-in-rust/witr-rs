use crate::core::models::SocketInfo;
use std::collections::HashMap;
use std::process::Command;

pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    let mut sockets = HashMap::new();

    if let Ok(output) = Command::new("netstat").args(["-ano"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue;
            }

            let proto = parts[0];
            let local_addr = parts[1];
            let remote_addr = if parts.len() > 2 { parts[2] } else { "" };

            let mut pid_str = "";
            let mut is_listening = false;
            let mut state = "UNKNOWN";

            if proto == "TCP" {
                if parts.len() >= 5 {
                    state = parts[3];
                    if state == "LISTENING" {
                        pid_str = parts[4];
                        is_listening = true;
                    }
                }
            } else if proto == "UDP" && parts.len() >= 4 {
                state = "LISTENING";
                pid_str = parts.last().unwrap();
                is_listening = true;
            }

            if is_listening {
                if let Some(port_part) = local_addr.rsplit(':').next() {
                    if let Ok(p) = port_part.parse::<u16>() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            let key = ((pid as u64) << 16) | (p as u64);

                            let info = SocketInfo::new(
                                p,
                                state.to_string(),
                                local_addr.to_string(),
                                remote_addr.to_string(),
                            );

                            sockets.insert(key, info);
                        }
                    }
                }
            }
        }
    }
    sockets
}

pub fn get_sockets_for_pid(target_pid: u32) -> Vec<u64> {
    let mut keys = Vec::new();

    if let Ok(output) = Command::new("netstat").args(["-ano"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue;
            }

            let proto = parts[0];
            let local_addr = parts[1];
            let pid_str = parts.last().unwrap();

            if let Ok(pid) = pid_str.parse::<u32>() {
                if pid == target_pid {
                    let valid = (proto == "TCP" && parts.len() >= 5 && parts[3] == "LISTENING")
                        || (proto == "UDP");

                    if valid {
                        if let Some(port_part) = local_addr.rsplit(':').next() {
                            if let Ok(p) = port_part.parse::<u16>() {
                                let key = ((pid as u64) << 16) | (p as u64);
                                keys.push(key);
                            }
                        }
                    }
                }
            }
        }
    }
    keys
}

pub fn get_socket_state(_pid: u32) -> HashMap<u64, String> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_key_generation() {
        let pid = 1234;
        let port = 8080;
        let key = ((pid as u64) << 16) | (port as u64);
        let expected = (1234u64 * 65536) + 8080;
        assert_eq!(key, expected);
    }
}
