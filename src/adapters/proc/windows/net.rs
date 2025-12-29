use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SocketInfo {
    pub port: u16,
    pub local_addr: String,
}

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
            
            let mut pid_str = "";
            let mut is_listening = false;

            if proto == "TCP" {
                if parts.len() >= 5 {
                    let state = parts[3];
                    if state == "LISTENING" {
                        pid_str = parts[4];
                        is_listening = true;
                    }
                }
            } else if proto == "UDP" && parts.len() >= 4 {
                 pid_str = parts.last().unwrap();
                 is_listening = true;
            }

            if is_listening {
                if let Some(port_part) = local_addr.rsplit(':').next() {
                    if let Ok(p) = port_part.parse::<u16>() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            let key = ((pid as u64) << 16) | (p as u64);
                            sockets.insert(
                                key,
                                SocketInfo {
                                    port: p,
                                    local_addr: local_addr.to_string(),
                                },
                            );
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
    fn test_parse_netstat_line() {
        let line = "  TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1760";
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts[0], "TCP");
        assert_eq!(parts[1], "0.0.0.0:135");
        assert_eq!(parts[3], "LISTENING");
        assert_eq!(parts[4], "1760");
    }

    #[test]
    fn test_key_generation() {
        let pid = 1234;
        let port = 8080;
        let key = ((pid as u64) << 16) | (port as u64);
        let expected = (1234u64 * 65536) + 8080;
        assert_eq!(key, expected);
    }
}
