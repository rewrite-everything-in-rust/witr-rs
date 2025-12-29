use std::process::Command;

pub fn get_user_name(uid: u32) -> Option<String> {
    if let Ok(output) = Command::new("id").arg("-un").arg(uid.to_string()).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_id_output(&output_str);
    }
    None
}

fn parse_id_output(output: &str) -> Option<String> {
    let trimmed = output.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id_output() {
        assert_eq!(parse_id_output("root\n"), Some("root".to_string()));
        assert_eq!(parse_id_output("user"), Some("user".to_string()));
        assert_eq!(parse_id_output(""), None);
        assert_eq!(parse_id_output("   \n"), None);
    }
}
