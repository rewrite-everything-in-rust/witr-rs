use std::collections::HashMap;
use std::fs;

pub struct SocketInfo {
    pub port: u16,
    pub local_addr: String,
}

pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    if let Ok(tcp) = fs::read_to_string("/proc/net/tcp") {
        return parse_tcp_file(&tcp);
    }
    HashMap::new()
}

fn parse_tcp_file(content: &str) -> HashMap<u64, SocketInfo> {
    let mut sockets = HashMap::new();
    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Some(local_addr) = parts.get(1) {
                let addr_parts: Vec<&str> = local_addr.split(':').collect();
                if addr_parts.len() == 2 {
                    if let Ok(port) = u16::from_str_radix(addr_parts[1], 16) {
                        if let Ok(inode) = parts[9].parse::<u64>() {
                            sockets.insert(
                                inode,
                                SocketInfo {
                                    port,
                                    local_addr: "0.0.0.0".to_string(), // In real implementation we might parse IP too
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

pub fn get_sockets_for_pid(pid: u32) -> Vec<u64> {
    let mut inodes = Vec::new();
    let fd_dir = format!("/proc/{}/fd", pid);

    if let Ok(entries) = fs::read_dir(&fd_dir) {
        for entry in entries.flatten() {
            if let Ok(link) = fs::read_link(entry.path()) {
                let link_str = link.to_string_lossy();
                if let Some(inode) = parse_socket_link(&link_str) {
                    inodes.push(inode);
                }
            }
        }
    }

    inodes
}

fn parse_socket_link(link: &str) -> Option<u64> {
    if link.starts_with("socket:[") {
        if let Some(inode_str) = link
            .strip_prefix("socket:[")
            .and_then(|s| s.strip_suffix(']'))
        {
            return inode_str.parse::<u64>().ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tcp_file() {
        let content = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode                                                     
   0: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 21623 1 0000000000000000 100 0 0 10 0
   1: 0100007F:0277 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 24159 1 0000000000000000 100 0 0 10 0
";
        let sockets = parse_tcp_file(content);
        assert_eq!(sockets.len(), 2);

        // 1F90 hex = 8080 decimal, inode 21623
        let s1 = sockets.get(&21623).unwrap();
        assert_eq!(s1.port, 8080);

        // 0277 hex = 631 decimal, inode 24159
        let s2 = sockets.get(&24159).unwrap();
        assert_eq!(s2.port, 631);
    }

    #[test]
    fn test_parse_socket_link() {
        assert_eq!(parse_socket_link("socket:[12345]"), Some(12345));
        assert_eq!(parse_socket_link("socket:[67890]"), Some(67890));
        assert_eq!(parse_socket_link("/dev/null"), None);
        assert_eq!(parse_socket_link("pipe:[111]"), None);
    }
}
