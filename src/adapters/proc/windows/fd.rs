pub fn get_fd_count(_pid: u32) -> Option<usize> {
    None
}

pub fn get_fd_limit(_pid: u32) -> Option<(u64, u64)> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fd_stub() {
        assert_eq!(get_fd_count(123), None);
        assert_eq!(get_fd_limit(123), None);
    }
}
