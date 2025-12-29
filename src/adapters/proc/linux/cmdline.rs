use std::fs;

pub fn get_cmdline(pid: u32) -> Vec<String> {
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    if let Ok(content) = fs::read(&cmdline_path) {
        return parse_cmdline(&content);
    }
    Vec::new()
}

fn parse_cmdline(content: &[u8]) -> Vec<String> {
    let mut args = Vec::new();
    // Split by null byte
    for chunk in content.split(|&b| b == 0) {
        if !chunk.is_empty() {
            if let Ok(arg) = String::from_utf8(chunk.to_vec()) {
                args.push(arg);
            }
        }
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmdline() {
        let content = b"prog\0arg1\0arg2\0";
        let args = parse_cmdline(content);
        assert_eq!(args, vec!["prog", "arg1", "arg2"]);

        // No nulls or empty
        let empty = b"";
        assert!(parse_cmdline(empty).is_empty());
    }
}
