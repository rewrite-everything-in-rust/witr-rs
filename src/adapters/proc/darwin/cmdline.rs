use std::process::Command;

pub fn get_cmdline(pid: u32) -> Vec<String> {
    if let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_ps_cmdline(&output_str);
    }
    Vec::new()
}

fn parse_ps_cmdline(output: &str) -> Vec<String> {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    
    trimmed
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ps_cmdline() {
        let output = "node index.js --dev\n";
        let args = parse_ps_cmdline(output);
        assert_eq!(args, vec!["node", "index.js", "--dev"]);

        assert!(parse_ps_cmdline("").is_empty());
    }
}
