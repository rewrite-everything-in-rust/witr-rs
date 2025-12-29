use crate::core::models::SocketInfo;
use std::collections::HashMap;

#[cfg(target_os = "linux")]
pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    use std::fs;

    let mut sockets = HashMap::new();

    if let Ok(tcp) = fs::read_to_string("/proc/net/tcp") {
        parse_proc_net(&tcp, &mut sockets, false);
    }

    if let Ok(tcp6) = fs::read_to_string("/proc/net/tcp6") {
        parse_proc_net(&tcp6, &mut sockets, true);
    }

    sockets
}

#[cfg(target_os = "linux")]
fn parse_proc_net(content: &str, sockets: &mut HashMap<u64, SocketInfo>, is_ipv6: bool) {
    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            continue;
        }

        let state = parts.get(3).and_then(|s| u32::from_str_radix(s, 16).ok());

        if let Some(local_addr) = parts.get(1) {
            let addr_parts: Vec<&str> = local_addr.split(':').collect();
            if addr_parts.len() != 2 {
                continue;
            }

            if let Ok(port) = u16::from_str_radix(addr_parts[1], 16) {
                if let Ok(inode) = parts[9].parse::<u64>() {
                    let ip_hex = addr_parts[0];
                    let address = if is_ipv6 {
                        parse_ipv6_hex(ip_hex)
                    } else {
                        parse_ipv4_hex(ip_hex)
                    };

                    let socket_state = match state {
                        Some(0x01) => "ESTABLISHED",
                        Some(0x02) => "SYN_SENT",
                        Some(0x03) => "SYN_RECV",
                        Some(0x06) => "TIME_WAIT",
                        Some(0x07) => "CLOSE",
                        Some(0x08) => "CLOSE_WAIT",
                        Some(0x0A) => "LISTEN",
                        _ => "UNKNOWN",
                    }
                    .to_string();

                    sockets.insert(
                        inode,
                        SocketInfo {
                            port,
                            state: socket_state,
                            local_addr: address.clone(),
                            remote_addr: String::new(),
                            explanation: String::new(),
                            workaround: String::new(),
                        },
                    );
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn parse_ipv4_hex(hex: &str) -> String {
    if hex.len() != 8 {
        return "0.0.0.0".to_string();
    }

    let bytes: Vec<u8> = (0..4)
        .rev()
        .filter_map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok())
        .collect();

    if bytes.len() == 4 {
        format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    } else {
        "0.0.0.0".to_string()
    }
}

#[cfg(target_os = "linux")]
fn parse_ipv6_hex(hex: &str) -> String {
    if hex.len() != 32 {
        return "::".to_string();
    }

    let parts: Vec<String> = (0..8)
        .rev()
        .filter_map(|i| {
            let idx = i * 4;
            if idx + 4 <= hex.len() {
                Some(format!(
                    "{:x}",
                    u16::from_str_radix(&hex[idx..idx + 4], 16).ok()?
                ))
            } else {
                None
            }
        })
        .collect();

    parts.join(":")
}

#[cfg(target_os = "windows")]
pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    use std::process::Command;

    let mut sockets = HashMap::new();

    if let Ok(output) = Command::new("netstat").args(["-ano", "-p", "TCP"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(4) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let state = parts.get(3).map(|s| s.to_string()).unwrap_or_default();

                if let Some(local) = parts.get(1) {
                    if let Some((addr, port_str)) = local.rsplit_once(':') {
                        if let Ok(port) = port_str.parse::<u16>() {
                            if let Some(pid_str) = parts.get(4) {
                                if let Ok(pid) = pid_str.parse::<u64>() {
                                    sockets.insert(
                                        pid,
                                        SocketInfo {
                                            port,
                                            state,
                                            local_addr: addr
                                                .trim_start_matches('[')
                                                .trim_end_matches(']')
                                                .to_string(),
                                            remote_addr: String::new(),
                                            explanation: String::new(),
                                            workaround: String::new(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sockets
}

#[cfg(target_os = "macos")]
pub fn get_listening_sockets() -> HashMap<u64, SocketInfo> {
    use std::process::Command;

    let mut sockets = HashMap::new();

    if let Ok(output) = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-nP"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                if let Ok(pid) = parts.get(1).and_then(|s| s.parse::<u64>().ok()) {
                    if let Some(addr_port) = parts.get(8) {
                        if let Some((addr, port_str)) = addr_port.rsplit_once(':') {
                            if let Ok(port) = port_str.parse::<u16>() {
                                sockets.insert(
                                    pid,
                                    SocketInfo {
                                        port,
                                        state: "LISTEN".to_string(),
                                        local_addr: addr
                                            .trim_start_matches('[')
                                            .trim_end_matches(']')
                                            .to_string(),
                                        remote_addr: String::new(),
                                        explanation: String::new(),
                                        workaround: String::new(),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    sockets
}

pub fn get_sockets_for_pid(pid: u32) -> Vec<u64> {
    super::fd::get_open_fds(pid)
}
