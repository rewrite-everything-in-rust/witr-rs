pub fn is_network_service(local_port: u16) -> bool {
    // If port is privileged (< 1024) likely a system service
    local_port < 1024
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_network_service() {
        assert!(is_network_service(80));
        assert!(is_network_service(22));
        assert!(!is_network_service(8080));
    }
}
