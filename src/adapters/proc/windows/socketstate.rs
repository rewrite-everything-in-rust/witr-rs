use std::collections::HashMap;
use std::process::Command;

pub fn get_socket_state(target_pid: u32) -> HashMap<u64, String> {
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
            let state = parts[3];
            let pid_str = parts[4];

            if proto == "TCP" {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    if pid == target_pid {
                        if let Some(port_part) = local_addr.rsplit(':').next() {
                            if let Ok(port) = port_part.parse::<u16>() {
                                let key = ((pid as u64) << 16) | (port as u64);
                                states.insert(key, state.to_string());
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
    #[test]
    fn test_parse_socket_state() {
        let line = "  TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1760";
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts[3], "LISTENING");
        assert_eq!(parts[4], "1760");
    }
}
