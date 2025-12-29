use crate::adapters::socketstate;
use crate::core::models::Process;
use crate::core::ports::{SystemError, SystemProvider};
use std::fs;
use std::path::PathBuf;
use sysinfo::{Pid, ProcessStatus, System};

pub struct RealSystem {
    sys: System,
}

impl RealSystem {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    fn get_git_info(&self, cwd: Option<&String>) -> (Option<String>, Option<String>) {
        let Some(cwd_str) = cwd else {
            return (None, None);
        };
        let mut current_dir = PathBuf::from(cwd_str);

        loop {
            let git_dir = current_dir.join(".git");
            if git_dir.exists() && git_dir.is_dir() {
                let repo_name = current_dir
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned());

                let head_path = git_dir.join("HEAD");
                let branch = if let Ok(contents) = fs::read_to_string(head_path) {
                    let contents = contents.trim();
                    if let Some(stripped) = contents.strip_prefix("ref: ") {
                        stripped.split('/').last().map(|s| s.to_string())
                    } else {
                        Some(contents.chars().take(7).collect())
                    }
                } else {
                    None
                };

                return (repo_name, branch);
            }

            if !current_dir.pop() {
                break;
            }
        }

        (None, None)
    }

    #[cfg(target_os = "windows")]
    fn get_service_info(&self, pid: u32) -> Option<String> {
        use std::process::Command;

        let output = Command::new("tasklist")
            .args(["/svc", "/fi", &format!("PID eq {}", pid)])
            .output()
            .ok()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(3) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let service_name = parts.get(2)?;
                if service_name != &"N/A" {
                    return Some(service_name.to_string());
                }
            }
        }
        None
    }

    #[cfg(target_os = "macos")]
    fn get_service_info(&self, pid: u32) -> Option<String> {
        use std::path::Path;
        use std::process::Command;

        let output = Command::new("launchctl").args(["list"]).output().ok()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let pid_str = pid.to_string();

        for line in output_str.lines() {
            if line.contains(&pid_str) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(label) = parts.last() {
                    return Some(label.to_string());
                }
            }
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    fn get_service_info(&self, pid: u32) -> Option<String> {
        use std::path::Path;
        use std::process::Command;

        let output = Command::new("systemctl")
            .arg("status")
            .arg(pid.to_string())
            .output()
            .ok()?;

        let out_str = String::from_utf8_lossy(&output.stdout);
        for line in out_str.lines() {
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

    fn get_container_info(&self, _pid: u32) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            let path = format!("/proc/{}/cgroup", _pid);
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("docker") {
                    return Some("docker".into());
                }
                if content.contains("containerd") {
                    return Some("containerd".into());
                }
                if content.contains("kubepods") {
                    return Some("kubernetes".into());
                }
            }
        }
        None
    }

    fn get_network_info(&self, pid: u32) -> (Vec<u16>, Vec<String>) {
        let sockets = socketstate::get_listening_sockets();

        #[cfg(target_os = "linux")]
        {
            let inodes = socketstate::get_sockets_for_pid(pid);
            let mut ports = Vec::new();
            let mut addrs = Vec::new();

            for inode in inodes {
                if let Some(socket) = sockets.get(&inode) {
                    ports.push(socket.port);
                    addrs.push(socket.local_addr.clone());
                }
            }

            (ports, addrs)
        }

        #[cfg(not(target_os = "linux"))]
        {
            if let Some(socket) = sockets.get(&(pid as u64)) {
                (vec![socket.port], vec![socket.local_addr.clone()])
            } else {
                (Vec::new(), Vec::new())
            }
        }
    }

    fn get_health_status(
        &self,
        _pid: u32,
        status: ProcessStatus,
        memory: u64,
        cpu_usage: f32,
        start_time: u64,
    ) -> String {
        match status {
            ProcessStatus::Zombie => return "zombie".to_string(),
            ProcessStatus::Stop => return "stopped".to_string(),
            _ => {}
        }

        if memory > 1024 * 1024 * 1024 {
            return "high-mem".to_string();
        }

        if cpu_usage > 80.0 {
            return "high-cpu".to_string();
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if now.saturating_sub(start_time) > 90 * 24 * 3600 {
            return "long-running".to_string();
        }

        "healthy".to_string()
    }

    fn detect_forked(&self, parent_pid: Option<u32>) -> String {
        match parent_pid {
            Some(1) | None => "not-forked".to_string(),
            Some(_) => "forked".to_string(),
        }
    }
}

impl SystemProvider for RealSystem {
    fn get_process_by_pid(&self, pid: u32) -> Result<Process, SystemError> {
        let pid_struct = Pid::from(pid as usize);

        if let Some(p) = self.sys.process(pid_struct) {
            let cwd = p.cwd().map(|c| c.to_string_lossy().into_owned());
            let (git_repo, git_branch) = self.get_git_info(cwd.as_ref());
            let container = self.get_container_info(pid);
            let service = self.get_service_info(pid);
            let (ports, bind_addrs) = self.get_network_info(pid);
            let health =
                self.get_health_status(pid, p.status(), p.memory(), p.cpu_usage(), p.start_time());
            let forked = self.detect_forked(p.parent().map(|p| p.as_u32()));

            Ok(Process {
                pid: p.pid().as_u32(),
                parent_pid: p.parent().map(|pid| pid.as_u32()),
                name: p.name().to_string_lossy().into_owned(),
                cmd: p
                    .cmd()
                    .iter()
                    .map(|s| s.to_string_lossy().into_owned())
                    .collect(),
                exe_path: p.exe().map(|path| path.to_string_lossy().into_owned()),
                uid: p.user_id().map(|u| u.to_string()),
                username: None,
                start_time: p.start_time(),
                cwd,
                git_repo,
                git_branch,
                container,
                service,
                ports,
                bind_addrs,
                health,
                forked,
                env: p
                    .environ()
                    .iter()
                    .map(|s| s.to_string_lossy().into_owned())
                    .collect(),
            })
        } else {
            Err(SystemError::ProcessNotFound(pid.to_string()))
        }
    }

    fn find_processes_by_name(&self, name_query: &str) -> Result<Vec<Process>, SystemError> {
        let mut matches = Vec::new();
        for (pid, p) in self.sys.processes() {
            let name = p.name().to_string_lossy();
            if name.contains(name_query) {
                let cwd = p.cwd().map(|c| c.to_string_lossy().into_owned());
                let (git_repo, git_branch) = self.get_git_info(cwd.as_ref());

                matches.push(Process {
                    pid: pid.as_u32(),
                    parent_pid: p.parent().map(|pid| pid.as_u32()),
                    name: name.into_owned(),
                    cmd: p
                        .cmd()
                        .iter()
                        .map(|s| s.to_string_lossy().into_owned())
                        .collect(),
                    exe_path: p.exe().map(|path| path.to_string_lossy().into_owned()),
                    uid: p.user_id().map(|u| u.to_string()),
                    username: None,
                    start_time: p.start_time(),
                    cwd,
                    git_repo,
                    git_branch,
                    container: None,
                    service: None,
                    ports: vec![],
                    bind_addrs: vec![],
                    health: "healthy".to_string(),
                    forked: "unknown".to_string(),
                    env: p
                        .environ()
                        .iter()
                        .map(|s| s.to_string_lossy().into_owned())
                        .collect(),
                });
            }
        }

        if matches.is_empty() {
            return Err(SystemError::ProcessNotFound(name_query.to_string()));
        }
        Ok(matches)
    }

    fn find_process_by_port(&self, port: u16) -> Result<Process, SystemError> {
        let sockets = socketstate::get_listening_sockets();

        #[cfg(target_os = "linux")]
        {
            let mut target_inode = None;

            for (inode, socket) in &sockets {
                if socket.port == port {
                    target_inode = Some(*inode);
                    break;
                }
            }

            let target_inode = target_inode.ok_or_else(|| {
                SystemError::ProcessNotFound(format!("No process listening on port {}", port))
            })?;

            for (_pid, _p) in self.sys.processes() {
                let pid_u32 = _pid.as_u32();
                let inodes = socketstate::get_sockets_for_pid(pid_u32);
                if inodes.contains(&target_inode) {
                    return self.get_process_by_pid(pid_u32);
                }
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            for (pid, socket) in &sockets {
                if socket.port == port {
                    return self.get_process_by_pid(*pid as u32);
                }
            }
        }

        Err(SystemError::ProcessNotFound(format!(
            "No process found listening on port {}",
            port
        )))
    }
}
