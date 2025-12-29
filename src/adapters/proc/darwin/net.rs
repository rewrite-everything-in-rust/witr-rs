use std::process::Command;
use std::collections::HashMap;

pub struct SocketInfo {
    pub port: u16,
    pub local_addr: String,
}

pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    if let Ok(output) = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-nP"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_listening_sockets(&output_str);
    }
    
    HashMap::new()
}

fn parse_listening_sockets(output_str: &str) -> HashMap<u64, SocketInfo> {
    let mut sockets = HashMap::new();
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let fd_str = parts[3].trim_end_matches(char::is_alphabetic);
            if let Ok(fd) = fd_str.parse::<u64>() {
                if let Some(addr_port) = parts.get(8) {
                    if let Some((addr, port_str)) = addr_port.rsplit_once(':') {
                        if let Ok(port) = port_str.parse::<u16>() {
                            sockets.insert(fd, SocketInfo {
                                port,
                                local_addr: addr.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    sockets
}

pub fn get_sockets_for_pid(pid: u32) -> Vec<u64> {
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-nP", "-iTCP"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_pid_sockets(&output_str);
    }
    
    Vec::new()
}

fn parse_pid_sockets(output_str: &str) -> Vec<u64> {
    let mut fds = Vec::new();
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let fd_str = parts[3].trim_end_matches(char::is_alphabetic);
            if let Ok(fd) = fd_str.parse::<u64>() {
                fds.push(fd);
            }
        }
    }
    fds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_listening_sockets() {
        let output = "COMMAND   PID USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
controlle 666 user   10u  IPv4 0x1234567890abcdef      0t0  TCP 127.0.0.1:8080 (LISTEN)
node      777 user   22u  IPv6 0xabcdef1234567890      0t0  TCP *:3000 (LISTEN)
";
        let sockets = parse_listening_sockets(output);
        
        assert_eq!(sockets.len(), 2);
        
        // Test fd 10
        let socket1 = sockets.get(&10).unwrap();
        assert_eq!(socket1.port, 8080);
        assert_eq!(socket1.local_addr, "127.0.0.1");

        // Test fd 22
        let socket2 = sockets.get(&22).unwrap();
        assert_eq!(socket2.port, 3000);
        assert_eq!(socket2.local_addr, "*");
    }

    #[test]
    fn test_parse_pid_sockets() {
        let output = "COMMAND   PID USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
process   123 user   5u   IPv4 0x1234567890abcdef      0t0  TCP 127.0.0.1:54321->127.0.0.1:5432 (ESTABLISHED)
process   123 user   8u   IPv4 0xabcdef1234567890      0t0  TCP 127.0.0.1:8080 (LISTEN)
";
        let fds = parse_pid_sockets(output);
        assert_eq!(fds.len(), 2);
        assert!(fds.contains(&5));
        assert!(fds.contains(&8));
    }
}
