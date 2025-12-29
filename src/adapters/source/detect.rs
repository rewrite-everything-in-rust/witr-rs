use crate::adapters::source::{container, cron, git, network, service, shell, supervisor};

#[derive(Debug, PartialEq)]
pub enum SourceType {
    Container(String),
    Service(String),
    Git(String),
    Cron,
    Supervisor,
    Shell,
    Network(u16),
    Unknown,
}

pub fn detect_source(pid: u32, comm: &str, cwd: Option<&String>, port: Option<u16>) -> SourceType {
    // Order of precedence:
    // 1. Container (highest isolation)
    if let Some(id) = container::detect_container(pid) {
        return SourceType::Container(id);
    }

    // 2. Service Manager (Systemd/Launchd)
    if let Some(svc) = service::get_service_info(pid) {
        return SourceType::Service(svc);
    }

    // 3. Git Repo (Code context)
    if let (Some(repo), _) = git::get_git_info(cwd) {
        return SourceType::Git(repo);
    }

    // 4. Others
    if cron::is_cron_process(pid) {
        return SourceType::Cron;
    }

    if supervisor::is_supervisor_process(pid) {
        return SourceType::Supervisor;
    }

    if shell::is_shell_process(comm) {
        return SourceType::Shell;
    }

    if let Some(p) = port {
        if network::is_network_service(p) {
            return SourceType::Network(p);
        }
    }

    SourceType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_type_enum() {
        let s = SourceType::Container("docker".into());
        match s {
            SourceType::Container(v) => assert_eq!(v, "docker"),
            _ => panic!("wrong type"),
        }
    }
}
