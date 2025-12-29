use std::fs;

pub fn get_username(uid: &str) -> Option<String> {
    if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
        return parse_passwd(&passwd, uid);
    }
    None
}

fn parse_passwd(content: &str, target_uid: &str) -> Option<String> {
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && parts[2] == target_uid {
            return Some(parts[0].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_passwd() {
        let content = "root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
user:x:1000:1000:User Name,,,:/home/user:/bin/bash";

        assert_eq!(parse_passwd(content, "0"), Some("root".to_string()));
        assert_eq!(parse_passwd(content, "1000"), Some("user".to_string()));
        assert_eq!(parse_passwd(content, "9999"), None);
    }
}
