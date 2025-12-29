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
                    return Path::new(path)
                        .file_name()
                        .map(|f| f.to_string_lossy().into_owned());
                }
            }
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
        assert_eq!(parse_systemctl_status(output), Some("nginx.service".to_string()));

        let invalid = "Loaded: loaded (/etc/init.d/apache2; generated)";
        assert_eq!(parse_systemctl_status(invalid), None);
    }
}
