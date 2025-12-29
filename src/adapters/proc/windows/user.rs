use std::process::Command;

pub fn get_process_owner(pid: u32) -> Option<String> {
    let output = Command::new("tasklist")
        .args(["/V", "/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();

    if line.is_empty() {
        return None;
    }

    let parts: Vec<&str> = line.split("\",\"").collect();

    if parts.len() < 7 {
        return None;
    }

    let user_part = parts[6].trim_matches('"');

    if user_part == "N/A" {
        None
    } else {
        Some(user_part.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_tasklist_user() {
        let line =
            r#""witr.exe","123","Console","1","5,000 K","Unknown","MyPC\User","0:00:00","MyTitle""#;
        let parts: Vec<&str> = line.split("\",\"").collect();
        let user = parts[6].trim_matches('"');
        assert_eq!(user, r"MyPC\User");
    }
}
