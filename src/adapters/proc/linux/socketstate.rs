use super::net::parse_ip_port;
use crate::core::models::SocketInfo;
use std::collections::HashMap;
use std::fs;

pub fn get_socket_state(_pid: u32) -> HashMap<u64, SocketInfo> {
    let mut states = HashMap::new();
    if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
        states.extend(parse_tcp_states(&content));
    }
    if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
        states.extend(parse_tcp_states(&content));
    }
    states
}

fn parse_tcp_states(content: &str) -> HashMap<u64, SocketInfo> {
    let mut states = HashMap::new();
    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Ok(inode) = parts[9].parse::<u64>() {
                let local_hex = parts[1];
                let remote_hex = parts[2];
                let state_hex = parts[3];

                let state_str = match state_hex {
                    "01" => "ESTABLISHED",
                    "02" => "SYN_SENT",
                    "03" => "SYN_RECV",
                    "04" => "FIN_WAIT1",
                    "05" => "FIN_WAIT2",
                    "06" => "TIME_WAIT",
                    "07" => "CLOSE",
                    "08" => "CLOSE_WAIT",
                    "09" => "LAST_ACK",
                    "0A" => "LISTEN",
                    "0B" => "CLOSING",
                    _ => "UNKNOWN",
                };

                let mut local = "0.0.0.0:0".to_string();
                let mut remote = "0.0.0.0:0".to_string();
                let mut port = 0;

                if let Some((ip, p)) = parse_ip_port(local_hex) {
                    local = format!("{}:{}", ip, p);
                    port = p;
                }

                if let Some((ip, p)) = parse_ip_port(remote_hex) {
                    remote = format!("{}:{}", ip, p);
                }

                let info = SocketInfo::new(port, state_str.to_string(), local, remote);

                states.insert(inode, info);
            }
        }
    }
    states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tcp_states() {
        let content = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode                                                     
   0: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 21623 1 0000000000000000 100 0 0 10 0
   1: 0100007F:0277 0100007F:1F90 01 00000000:00000000 00:00000000 00000000  1000        0 24159 1 0000000000000000 100 0 0 10 0
";
        let states = parse_tcp_states(content);

        // Inode 21623, State 0A -> LISTEN
        if let Some(s) = states.get(&21623) {
            assert_eq!(s.state, "LISTEN");
            assert_eq!(s.port, 8080); // 1F90 hex -> 8080
        }

        // Inode 24159, State 01 -> ESTABLISHED
        if let Some(s) = states.get(&24159) {
            assert_eq!(s.state, "ESTABLISHED");
            assert_eq!(s.port, 631); // 0277 hex -> 631
        }
    }
}
