use std::process::Command;

pub fn get_open_files(pid: u32) -> Vec<String> {
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-Fn"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_lsof_files(&output_str);
    }
    
    Vec::new()
}

fn parse_lsof_files(output_str: &str) -> Vec<String> {
    let mut files = Vec::new();
    for line in output_str.lines() {
        if let Some(stripped) = line.strip_prefix('n') {
            files.push(stripped.to_string());
        }
    }
    files
}

pub fn get_file_limit(pid: u32) -> Option<(u64, u64)> {
    if let Ok(output) = Command::new("ulimit").args(["-n"]).output() {
        let limit_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Ok(limit) = limit_str.parse::<u64>() {
            let current = get_open_files(pid).len() as u64;
            return Some((current, limit));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lsof_files() {
        let output = "p1234
f5
n/usr/lib/libSystem.B.dylib
f6
n/dev/null
f8
n/Users/user/project/file.txt
";
        let files = parse_lsof_files(output);
        assert_eq!(files.len(), 3);
        assert_eq!(files[0], "/usr/lib/libSystem.B.dylib");
        assert_eq!(files[1], "/dev/null");
        assert_eq!(files[2], "/Users/user/project/file.txt");
    }
}
