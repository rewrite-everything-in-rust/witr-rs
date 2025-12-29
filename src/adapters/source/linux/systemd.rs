use std::path::Path;
use std::process::Command;

pub fn get_systemd_service(pid: u32) -> Option<String> {
    if let Ok(output) = Command::new("systemctl")
        .arg("status")
        .arg(pid.to_string())
        .output()
    {
        let out_str = String::from_utf8_lossy(&output.stdout);
        return parse_systemctl_status(&out_str);
    }
    None
}

fn parse_systemctl_status(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.trim().starts_with("Loaded:") && line.contains(".service") {
            if let Some(start) = line.find("(/") {
                if let Some(end) = line[start..].find(";") {
                    let path = &line[start + 1..start + end];
                    let service_name = Path::new(path)
                        .file_name()
                        .map(|f| f.to_string_lossy().into_owned());

                    if let Some(name) = &service_name {
                        if name.starts_with("user@") {
                            return None;
                        }
                    }
                    return service_name;
                }
            }
        }
    }
    None
}

pub fn get_restart_count(service_name: &str) -> Option<u32> {
    if let Ok(output) = Command::new("systemctl")
        .args(["show", "-p", "NRestarts", "--value", service_name])
        .output()
    {
        let out_str = String::from_utf8_lossy(&output.stdout);
        return parse_restart_count(&out_str);
    }
    None
}

fn parse_restart_count(output: &str) -> Option<u32> {
    output.trim().parse::<u32>().ok()
}

pub fn get_fragment_path(service_name: &str) -> Option<String> {
    if let Ok(output) = Command::new("systemctl")
        .args(["show", "-p", "FragmentPath", "--value", service_name])
        .output()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        let path = s.trim();
        if !path.is_empty() && path != "/dev/null" {
            return Some(path.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_systemctl_status() {
        let output = r#"
● nginx.service - A high performance web server and a reverse proxy server
   Loaded: loaded (/lib/systemd/system/nginx.service; enabled; vendor preset: enabled)
   Active: active (running) since Mon 2023-01-16 ...
"#;
        assert_eq!(
            parse_systemctl_status(output),
            Some("nginx.service".to_string())
        );

        let invalid = "Loaded: loaded (/etc/init.d/apache2; generated)";
        assert_eq!(parse_systemctl_status(invalid), None);

        let user_service = r#"
   Loaded: loaded (/lib/systemd/system/user@.service; static; vendor preset: enabled)
"#;
        assert_eq!(parse_systemctl_status(user_service), None);
    }

    #[test]
    fn test_parse_restart_count() {
        assert_eq!(parse_restart_count("5\n"), Some(5));
        assert_eq!(parse_restart_count("0"), Some(0));
        assert_eq!(parse_restart_count("invalid"), None);
        assert_eq!(parse_restart_count("  10  "), Some(10));
    }
}
