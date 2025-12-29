use super::{Process, Source, SourceType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionResult {
    pub process: Process,
    pub restart_count: u32,
    pub ancestry: Vec<Process>,
    pub source: Source,
    pub warnings: Vec<String>,
}

impl InspectionResult {
    pub fn new(process: Process, ancestry: Vec<Process>) -> Self {
        let source = Self::detect_source(&process, &ancestry);
        let warnings = Self::generate_warnings(&process, &ancestry);
        let restart_count = 0;

        Self {
            process,
            restart_count,
            ancestry,
            source,
            warnings,
        }
    }

    fn detect_source(process: &Process, ancestry: &[Process]) -> Source {
        if let Some(service) = &process.service {
            if service.ends_with(".service") {
                return Source {
                    source_type: SourceType::Systemd,
                    name: Some(service.clone()),
                };
            }
            if service.contains("com.") {
                return Source {
                    source_type: SourceType::Launchd,
                    name: Some(service.clone()),
                };
            }
        }

        if process.container.is_some() {
            return Source {
                source_type: SourceType::Docker,
                name: process.container.clone(),
            };
        }

        for parent in ancestry.iter().take(ancestry.len().saturating_sub(1)) {
            if parent.name.contains("pm2") {
                return Source {
                    source_type: SourceType::PM2,
                    name: Some("pm2".to_string()),
                };
            }
            if parent.name.contains("supervisord") {
                return Source {
                    source_type: SourceType::Supervisor,
                    name: Some("supervisord".to_string()),
                };
            }
            if parent.name.contains("cron") || parent.name.contains("CRON") {
                return Source {
                    source_type: SourceType::Cron,
                    name: Some("cron".to_string()),
                };
            }
        }

        if process.parent_pid == Some(1) || process.parent_pid.is_none() {
            Source {
                source_type: SourceType::System,
                name: None,
            }
        } else {
            Source {
                source_type: SourceType::Manual,
                name: None,
            }
        }
    }

    fn generate_warnings(process: &Process, ancestry: &[Process]) -> Vec<String> {
        let mut warnings = Vec::new();

        if process.health != "healthy" {
            warnings.push(format!("Process is {}", process.health));
        }

        if process.uid == Some("0".to_string()) {
            warnings.push("Running as root".to_string());
        }

        for (port, addr) in process.ports.iter().zip(&process.bind_addrs) {
            if addr.starts_with("0.0.0.0") || addr == "::" {
                warnings.push(format!("Listening publicly on {}:{}", addr, port));
            }
        }

        if let Some(exe) = &process.exe_path {
            if exe.contains("(deleted)") {
                warnings.push(format!(
                    "BINARY DELETED: Executable file has been deleted ({})",
                    exe
                ));
            }
            if exe.starts_with("/tmp") || exe.starts_with("/var/tmp") || exe.starts_with("/dev/shm")
            {
                warnings.push(format!(
                    "SUSPICIOUS LOCATION: Running from temporary directory ({})",
                    exe
                ));
            }
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if now.saturating_sub(process.start_time) > 90 * 24 * 3600 {
            warnings.push("Process has been running for over 90 days".to_string());
        }

        // Reverse Shell Detection
        let shell_names = [
            "sh",
            "bash",
            "zsh",
            "dash",
            "ash",
            "csh",
            "ksh",
            "powershell",
            "pwsh",
            "cmd",
            "cmd.exe",
        ];
        let is_shell = shell_names.contains(&process.name.as_str())
            || (process.name.len() < 5 && process.name.ends_with("sh"));

        if is_shell && ancestry.len() >= 2 {
            let parent = &ancestry[1];
            let web_servers = [
                "nginx", "httpd", "apache", "node", "php", "gunicorn", "uwsgi", "tomcat", "java",
            ];

            if web_servers.iter().any(|s| parent.name.contains(s)) {
                warnings.push(format!(
                    "POTENTIAL REVERSE SHELL: Shell spawned by web server process '{}'",
                    parent.name
                ));
            }
        }

        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_process(pid: u32, name: &str) -> Process {
        Process {
            pid,
            parent_pid: Some(1),
            name: name.to_string(),
            cmd: vec![name.to_string()],
            exe_path: Some(format!("/usr/bin/{}", name)),
            uid: Some("1000".to_string()),
            username: Some("user".to_string()),
            start_time: 1000,
            cwd: Some("/home/user".to_string()),
            git_repo: None,
            git_branch: None,
            container: None,
            service: None,
            service_file: None,
            ports: vec![],
            bind_addrs: vec![],
            port_states: vec![],
            restart_count: None,
            health: "healthy".to_string(),
            forked: "forked".to_string(),
            env: vec![],
        }
    }

    #[test]
    fn test_source_detection_systemd() {
        let mut process = mock_process(100, "myapp");
        process.service = Some("myapp.service".to_string());
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert_eq!(result.source.source_type, SourceType::Systemd);
    }

    #[test]
    fn test_source_detection_docker() {
        let mut process = mock_process(100, "myapp");
        process.container = Some("docker".to_string());
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert_eq!(result.source.source_type, SourceType::Docker);
    }

    #[test]
    fn test_warnings_root() {
        let mut process = mock_process(100, "myapp");
        process.uid = Some("0".to_string());
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert!(result.warnings.iter().any(|w| w.contains("root")));
    }

    #[test]
    fn test_warnings_public_port() {
        let mut process = mock_process(100, "myapp");
        process.ports = vec![8080];
        process.bind_addrs = vec!["0.0.0.0".to_string()];
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert!(result.warnings.iter().any(|w| w.contains("public")));
    }

    #[test]
    fn test_warnings_deleted_binary() {
        let mut process = mock_process(100, "malware");
        process.exe_path = Some("/usr/bin/malware (deleted)".to_string());
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert!(result.warnings.iter().any(|w| w.contains("BINARY DELETED")));
    }

    #[test]
    fn test_warnings_suspicious_location() {
        let mut process = mock_process(100, "miner");
        process.exe_path = Some("/tmp/.X11-unix/miner".to_string());
        let ancestry = vec![process.clone()];
        let result = InspectionResult::new(process, ancestry);
        assert!(result
            .warnings
            .iter()
            .any(|w| w.contains("SUSPICIOUS LOCATION")));
    }

    #[test]
    fn test_warnings_reverse_shell() {
        let parent = mock_process(50, "nginx");
        let mut child = mock_process(100, "bash");
        child.parent_pid = Some(50);

        // Ancestry: [child, parent]
        let ancestry = vec![child.clone(), parent];
        let result = InspectionResult::new(child, ancestry);
        assert!(result.warnings.iter().any(|w| w.contains("REVERSE SHELL")));
    }
}
