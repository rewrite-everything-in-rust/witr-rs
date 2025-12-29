use std::fs;

pub fn get_boot_time() -> u64 {
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        return parse_boot_time(&content);
    }
    0
}

fn parse_boot_time(content: &str) -> u64 {
    for line in content.lines() {
        if line.starts_with("btime ") {
            if let Some(time_str) = line.split_whitespace().nth(1) {
                if let Ok(time) = time_str.parse::<u64>() {
                    return time;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boot_time() {
        let content = "cpu 123 456\nbtime 1673822939\nintr 123";
        assert_eq!(parse_boot_time(content), 1673822939);
        assert_eq!(parse_boot_time("invalid"), 0);
    }
}
