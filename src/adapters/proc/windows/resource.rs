use std::collections::HashMap;

pub fn get_resource_limits(_pid: u32) -> HashMap<String, String> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_limits_stub() {
        let limits = get_resource_limits(1234);
        assert!(limits.is_empty());
    }
}
