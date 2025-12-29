use std::process::Command;

pub fn get_open_fds(pid: u32) -> Vec<u64> {
    if let Ok(output) = Command::new("lsof")
        .args(["-p", &pid.to_string(), "-Ff"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_lsof_fds(&output_str);
    }

    Vec::new()
}

fn parse_lsof_fds(output_str: &str) -> Vec<u64> {
    let mut fds = Vec::new();
    for line in output_str.lines() {
        if let Some(stripped) = line.strip_prefix('f') {
            if let Ok(fd) = stripped.parse::<u64>() {
                fds.push(fd);
            }
        }
    }
    fds
}

pub fn count_open_files(pid: u32) -> usize {
    get_open_fds(pid).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lsof_fds() {
        let output = "p1234
f0
f1
f2
";
        let fds = parse_lsof_fds(output);
        assert_eq!(fds.len(), 3);
        assert_eq!(fds, vec![0, 1, 2]);
    }
}
