use crate::adapters::proc as network;
use crate::adapters::source;
use crate::core::models::Process;
use crate::core::ports::{SystemError, SystemProvider};
use std::cell::RefCell;
use sysinfo::{Pid, ProcessesToUpdate, System};

#[derive(Default)]
pub struct RealSystem {
    sys: RefCell<System>,
}

impl RealSystem {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys: RefCell::new(sys),
        }
    }

    fn get_network_info(
        &self,
        pid: u32,
    ) -> (
        Vec<u16>,
        Vec<String>,
        Vec<String>,
        Vec<crate::core::models::SocketInfo>,
    ) {
        let mut ports = Vec::new();
        let mut addrs = Vec::new();
        let mut states = Vec::new();
        let mut sockets_list = Vec::new();

        let socket_map = network::get_socket_state(pid);

        for (_, info) in socket_map {
            ports.push(info.port);
            addrs.push(info.local_addr.clone());
            states.push(info.state.clone());
            sockets_list.push(info);
        }

        // Dedup and sort
        ports.sort();
        ports.dedup();

        (ports, addrs, states, sockets_list)
    }
}

impl SystemProvider for RealSystem {
    fn get_process_by_pid(&self, pid: u32) -> Result<Process, SystemError> {
        let sys_pid = Pid::from_u32(pid);
        let mut sys = self.sys.borrow_mut();
        sys.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);

        let process = sys
            .process(sys_pid)
            .ok_or_else(|| SystemError::ProcessNotFound(format!("PID {} not found", pid)))?;

        let parent_pid = process.parent().map(|p| p.as_u32());
        let (ports, bind_addrs, port_states, sockets) = self.get_network_info(pid);
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

        let mut final_restart_count = service_name
            .as_ref()
            .and_then(|s| source::get_service_restart_count(s));

        if final_restart_count.is_none() && container.as_deref() == Some("docker") {
            if let Some(id) = source::get_container_id(pid) {
                final_restart_count = source::get_docker_restart_count(&id);
            }
        }

        Ok(Process {
            pid,
            parent_pid,
            name: process.name().to_string_lossy().to_string(),
            cmd: process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            exe_path: process.exe().map(|p| p.display().to_string()),
            uid: process.user_id().map(|u| u.to_string()),
            username: None,
            start_time: process.start_time(),
            cwd: process.cwd().map(|p| p.display().to_string()),
            git_repo,
            git_branch,
            container,
            service: service_name.clone(),
            ports,
            bind_addrs,
            port_states,
            sockets,
            restart_count: final_restart_count,
            service_file: service_name
                .as_ref()
                .and_then(|s| source::get_service_file(s)),
            health: health_status,
            forked,
            env: process
                .environ()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
        })
    }

    fn find_processes_by_name(&self, name_query: &str) -> Result<Vec<Process>, SystemError> {
        let mut results = Vec::new();
        let name_lower = name_query.to_lowercase();
        let sys = self.sys.borrow();

        for (sys_pid, process) in sys.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();
            if process_name.contains(&name_lower) {
                let pid = sys_pid.as_u32();
                let parent_pid = process.parent().map(|p| p.as_u32());
                let (ports, bind_addrs, port_states, sockets) = self.get_network_info(pid);
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

                let mut final_restart_count = service_name
                    .as_ref()
                    .and_then(|s| source::get_service_restart_count(s));

                if final_restart_count.is_none() && container.as_deref() == Some("docker") {
                    if let Some(id) = source::get_container_id(pid) {
                        final_restart_count = source::get_docker_restart_count(&id);
                    }
                }

                results.push(Process {
                    pid,
                    parent_pid,
                    name: process.name().to_string_lossy().to_string(),
                    cmd: process
                        .cmd()
                        .iter()
                        .map(|s| s.to_string_lossy().to_string())
                        .collect(),
                    exe_path: process.exe().map(|p| p.display().to_string()),
                    uid: process.user_id().map(|u| u.to_string()),
                    username: None,
                    start_time: process.start_time(),
                    cwd: process.cwd().map(|p| p.display().to_string()),
                    git_repo,
                    git_branch,
                    container,
                    service: service_name.clone(),
                    ports,
                    bind_addrs,
                    port_states,
                    sockets,
                    restart_count: final_restart_count,
                    service_file: service_name
                        .as_ref()
                        .and_then(|s| source::get_service_file(s)),
                    health: health_status,
                    forked,
                    env: process
                        .environ()
                        .iter()
                        .map(|s| s.to_string_lossy().to_string())
                        .collect(),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                });
            }
        }

        if results.is_empty() {
            Err(SystemError::ProcessNotFound(format!(
                "No processes matching '{}'",
                name_query
            )))
        } else {
            Ok(results)
        }
    }

    fn find_process_by_port(&self, port: u16) -> Result<Process, SystemError> {
        let sockets = network::get_listening_sockets();
        let sys = self.sys.borrow();

        for (fd, socket) in &sockets {
            if socket.port == port {
                for (sys_pid, process) in sys.processes() {
                    let pid = sys_pid.as_u32();
                    let fds = network::get_sockets_for_pid(pid);

                    if fds.contains(fd) {
                        let parent_pid = process.parent().map(|p| p.as_u32());
                        let (ports, bind_addrs, port_states, sockets) = self.get_network_info(pid);
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

                        let mut final_restart_count = service_name
                            .as_ref()
                            .and_then(|s| source::get_service_restart_count(s));

                        if final_restart_count.is_none() && container.as_deref() == Some("docker") {
                            if let Some(id) = source::get_container_id(pid) {
                                final_restart_count = source::get_docker_restart_count(&id);
                            }
                        }

                        return Ok(Process {
                            pid,
                            parent_pid,
                            name: process.name().to_string_lossy().to_string(),
                            cmd: process
                                .cmd()
                                .iter()
                                .map(|s| s.to_string_lossy().to_string())
                                .collect(),
                            exe_path: process.exe().map(|p| p.display().to_string()),
                            uid: process.user_id().map(|u| u.to_string()),
                            username: None,
                            start_time: process.start_time(),
                            cwd: process.cwd().map(|p| p.display().to_string()),
                            git_repo,
                            git_branch,
                            container,
                            service: service_name.clone(),
                            ports,
                            bind_addrs,
                            port_states,
                            sockets,
                            restart_count: final_restart_count,
                            service_file: service_name
                                .as_ref()
                                .and_then(|s| source::get_service_file(s)),
                            health: health_status,
                            forked,
                            env: process
                                .environ()
                                .iter()
                                .map(|s| s.to_string_lossy().to_string())
                                .collect(),
                            cpu_usage: process.cpu_usage(),
                            memory_usage: process.memory(),
                        });
                    }
                }
            }
        }

        Err(SystemError::ProcessNotFound(format!(
            "No process found on port {}",
            port
        )))
    }

    fn get_all_pids(&self) -> Result<Vec<u32>, SystemError> {
        Ok(self
            .sys
            .borrow()
            .processes()
            .keys()
            .map(|pid| pid.as_u32())
            .collect())
    }
}
