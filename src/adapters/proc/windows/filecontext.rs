use std::collections::HashMap;

pub fn get_file_context(_pid: u32) -> HashMap<String, String> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_context_stub() {
        let ctx = get_file_context(1234);
        assert!(ctx.is_empty());
    }
}
