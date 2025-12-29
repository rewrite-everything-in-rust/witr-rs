use std::collections::HashMap;
use std::fs;

pub struct SocketInfo {
    pub port: u16,
    pub local_addr: String,
}

pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    let mut sockets = HashMap::new();

    if let Ok(tcp) = fs::read_to_string("/proc/net/tcp") {
        sockets.extend(parse_tcp_file(&tcp));
    }

    if let Ok(tcp6) = fs::read_to_string("/proc/net/tcp6") {
        sockets.extend(parse_tcp_file(&tcp6));
    }

    sockets
}

fn parse_tcp_file(content: &str) -> HashMap<u64, SocketInfo> {
    let mut sockets = HashMap::new();
    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Some(local_addr) = parts.get(1) {
                if let Some((ip, port)) = parse_ip_port(local_addr) {
                    if let Ok(inode) = parts[9].parse::<u64>() {
                        sockets.insert(
                            inode,
                            SocketInfo {
                                port,
                                local_addr: ip.to_string(),
                            },
                        );
                    }
                }
            }
        }
    }
    sockets
}

fn parse_ip_port(hex_str: &str) -> Option<(std::net::IpAddr, u16)> {
    let (ip_hex, port_hex) = hex_str.rsplit_once(':')?;
    let port = u16::from_str_radix(port_hex, 16).ok()?;

    let ip = if ip_hex.len() == 8 {
        // IPv4: Little-endian
        let val = u32::from_str_radix(ip_hex, 16).ok()?;
        std::net::IpAddr::V4(std::net::Ipv4Addr::from(val.to_le_bytes()))
    } else if ip_hex.len() == 32 {
        // IPv6: 4 x 32-bit little-endian words
        let mut octets = [0u8; 16];
        for i in 0..4 {
            let start = i * 8;
            let end = start + 8;
            let word_hex = &ip_hex[start..end];
            let word = u32::from_str_radix(word_hex, 16).ok()?;
            let bytes = word.to_le_bytes();
            octets[i * 4] = bytes[0];
            octets[i * 4 + 1] = bytes[1];
            octets[i * 4 + 2] = bytes[2];
            octets[i * 4 + 3] = bytes[3];
        }
        std::net::IpAddr::V6(std::net::Ipv6Addr::from(octets))
    } else {
        return None;
    };

    Some((ip, port))
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
        // IPv4 example
        let content_v4 = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode                                                     
   0: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 21623 1 0000000000000000 100 0 0 10 0
   1: 0100007F:0277 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 24159 1 0000000000000000 100 0 0 10 0
";
        let sockets = parse_tcp_file(content_v4);
        assert_eq!(sockets.len(), 2);

        // 0.0.0.0:8080
        let s1 = sockets.get(&21623).unwrap();
        assert_eq!(s1.port, 8080);
        assert_eq!(s1.local_addr, "0.0.0.0");

        // 127.0.0.1:631 (0100007F -> 7F 00 00 01 little endian = 127.0.0.1)
        let s2 = sockets.get(&24159).unwrap();
        assert_eq!(s2.port, 631);
        assert_eq!(s2.local_addr, "127.0.0.1");
    }

    #[test]
    fn test_parse_ipv6() {
        // [::1]:631 in proc/net/tcp6 format
        // IPv6 ::1 is 0000...0001
        // Hex: 00000000 00000000 00000000 01000000 (4 little endian words)
        // 01000000 (LE) -> 00 00 00 01 (BE)
        let line = "   0: 00000000000000000000000001000000:0277 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 12345 1 0000000000000000 100 0 0 10 0";

        let sockets = parse_tcp_file(&format!("header\n{}", line));
        let s = sockets.get(&12345).unwrap();
        assert_eq!(s.port, 631);
        assert_eq!(s.local_addr, "::1");
    }

    #[test]
    fn test_parse_socket_link() {
        assert_eq!(parse_socket_link("socket:[12345]"), Some(12345));
        assert_eq!(parse_socket_link("socket:[67890]"), Some(67890));
        assert_eq!(parse_socket_link("/dev/null"), None);
        assert_eq!(parse_socket_link("pipe:[111]"), None);
    }
}
