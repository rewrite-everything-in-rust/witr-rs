use crate::adapters::network;
use crate::adapters::source;
use crate::core::models::Process;
use crate::core::ports::{SystemError, SystemProvider};
use sysinfo::{Pid, System};

#[derive(Default)]
pub struct RealSystem {
    sys: System,
}

impl RealSystem {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    fn get_network_info(&self, pid: u32) -> (Vec<u16>, Vec<String>) {
        let mut ports = Vec::new();
        let mut addrs = Vec::new();

        let sockets = network::get_listening_sockets();
        let fds = network::get_sockets_for_pid(pid);

        for fd in fds {
            if let Some(socket) = sockets.get(&fd) {
                ports.push(socket.port);
                addrs.push(socket.local_addr.clone());
            }
        }

        (ports, addrs)
    }
}

impl SystemProvider for RealSystem {
    fn get_process_by_pid(&self, pid: u32) -> Result<Process, SystemError> {
        let sys_pid = Pid::from_u32(pid);
        let process = self
            .sys
            .process(sys_pid)
            .ok_or_else(|| SystemError::ProcessNotFound(format!("PID {} not found", pid)))?;

        let parent_pid = process.parent().map(|p| p.as_u32());
        let (ports, bind_addrs) = self.get_network_info(pid);
        let cwd_string = process.cwd().map(|p| p.display().to_string());
        let (git_repo, git_branch) = source::get_git_info(cwd_string.as_ref());
        let service_name = source::get_service_info(pid);
        let container = source::detect_container(pid);
        let health_status = source::get_health_status(
            pid,
            process.status(),
            process.memory(),
            process.cpu_usage(),
            process.start_time(),
        );
        let forked = source::detect_forked(parent_pid);

        Ok(Process {
            pid,
            parent_pid,
            name: process.name().to_string_lossy().to_string(),
            cmd: process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect(),
            exe_path: process.exe().map(|p| p.display().to_string()),
            uid: process.user_id().map(|u| u.to_string()),
            username: None,
            start_time: process.start_time(),
            cwd: process.cwd().map(|p| p.display().to_string()),
            git_repo,
            git_branch,
            container,
            service: service_name,
            ports,
            bind_addrs,
            health: health_status,
            forked,
            env: process.environ().iter().map(|s| s.to_string_lossy().to_string()).collect(),
        })
    }

    fn find_processes_by_name(&self, name_query: &str) -> Result<Vec<Process>, SystemError> {
        let mut results = Vec::new();
        let name_lower = name_query.to_lowercase();

        for (sys_pid, process) in self.sys.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();
            if process_name.contains(&name_lower) {
                let pid = sys_pid.as_u32();
                let parent_pid = process.parent().map(|p| p.as_u32());
                let (ports, bind_addrs) = self.get_network_info(pid);
                let cwd_string = process.cwd().map(|p| p.display().to_string());
                let (git_repo, git_branch) = source::get_git_info(cwd_string.as_ref());
                let service_name = source::get_service_info(pid);
                let container = source::detect_container(pid);
                let health_status = source::get_health_status(
                    pid,
                    process.status(),
                    process.memory(),
                    process.cpu_usage(),
                    process.start_time(),
                );
                let forked = source::detect_forked(parent_pid);

                results.push(Process {
                    pid,
                    parent_pid,
                    name: process.name().to_string_lossy().to_string(),
                    cmd: process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                    exe_path: process.exe().map(|p| p.display().to_string()),
                    uid: process.user_id().map(|u| u.to_string()),
                    username: None,
                    start_time: process.start_time(),
                    cwd: process.cwd().map(|p| p.display().to_string()),
                    git_repo,
                    git_branch,
                    container,
                    service: service_name,
                    ports,
                    bind_addrs,
                    health: health_status,
                    forked,
                    env: process.environ().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                });
            }
        }

        if results.is_empty() {
            Err(SystemError::ProcessNotFound(format!("No processes matching '{}'", name_query)))
        } else {
            Ok(results)
        }
    }

    fn find_process_by_port(&self, port: u16) -> Result<Process, SystemError> {
        let sockets = network::get_listening_sockets();

        for (fd, socket) in &sockets {
            if socket.port == port {
                for (sys_pid, process) in self.sys.processes() {
                    let pid = sys_pid.as_u32();
                    let fds = network::get_sockets_for_pid(pid);
                    
                    if fds.contains(fd) {
                        let parent_pid = process.parent().map(|p| p.as_u32());
                        let (ports, bind_addrs) = self.get_network_info(pid);
                        let cwd_string = process.cwd().map(|p| p.display().to_string());
                        let (git_repo, git_branch) = source::get_git_info(cwd_string.as_ref());
                        let service_name = source::get_service_info(pid);
                        let container = source::detect_container(pid);
                        let health_status = source::get_health_status(
                            pid,
                            process.status(),
                            process.memory(),
                            process.cpu_usage(),
                            process.start_time(),
                        );
                        let forked = source::detect_forked(parent_pid);

                        return Ok(Process {
                            pid,
                            parent_pid,
                            name: process.name().to_string_lossy().to_string(),
                            cmd: process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                            exe_path: process.exe().map(|p| p.display().to_string()),
                            uid: process.user_id().map(|u| u.to_string()),
                            username: None,
                            start_time: process.start_time(),
                            cwd: process.cwd().map(|p| p.display().to_string()),
                            git_repo,
                            git_branch,
                            container,
                            service: service_name,
                            ports,
                            bind_addrs,
                            health: health_status,
                            forked,
                            env: process.environ().iter().map(|s| s.to_string_lossy().to_string()).collect(),
                        });
                    }
                }
            }
        }

        Err(SystemError::ProcessNotFound(format!("No process found on port {}", port)))
    }
}
