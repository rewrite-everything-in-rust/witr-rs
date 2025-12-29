use std::process::Command;

pub fn get_boot_time() -> u64 {
    if let Ok(output) = Command::new("sysctl")
        .args(["-n", "kern.boottime"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_boottime(&output_str);
    }
    0
}

fn parse_boottime(output: &str) -> u64 {
    if let Some(start) = output.find("sec = ") {
        let rest = &output[start + 6..];
        if let Some(end) = rest.find(',') {
            if let Ok(time) = rest[..end].parse::<u64>() {
                return time;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boottime() {
        let output = "{ sec = 1673822939, usec = 0 } Mon Jan 16 09:28:59 2023";
        assert_eq!(parse_boottime(output), 1673822939);
        assert_eq!(parse_boottime("invalid"), 0);
    }
}
