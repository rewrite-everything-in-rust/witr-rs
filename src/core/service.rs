use crate::core::models::Process;
use crate::core::ports::{SystemError, SystemProvider};
use std::sync::Arc;

pub struct WitrService<S: SystemProvider> {
    sys: Arc<S>,
}

impl<S: SystemProvider> WitrService<S> {
    pub fn new(sys: S) -> Self {
        Self { sys: Arc::new(sys) }
    }

    pub fn inspect_pid(&self, pid: u32) -> Result<Process, SystemError> {
        self.sys.get_process_by_pid(pid)
    }

    pub fn inspect_name(&self, name: &str) -> Result<Vec<Process>, SystemError> {
        self.sys.find_processes_by_name(name)
    }

    pub fn inspect_port(&self, port: u16) -> Result<Process, SystemError> {
        self.sys.find_process_by_port(port)
    }

    pub fn inspect_all(&self) -> Result<Vec<crate::core::models::InspectionResult>, SystemError> {
        let pids = self.sys.get_all_pids()?;
        let mut results = Vec::new();
        for pid in pids {
            if let Ok(res) = self.get_inspection(pid) {
                if !res.warnings.is_empty() {
                    results.push(res);
                }
            }
        }
        Ok(results)
    }

    pub fn get_inspection(
        &self,
        pid: u32,
    ) -> Result<crate::core::models::InspectionResult, SystemError> {
        let process = self.inspect_pid(pid)?;
        let ancestry = self.get_ancestry(pid)?;
        Ok(crate::core::models::InspectionResult::new(
            process, ancestry,
        ))
    }

    pub fn get_ancestry(&self, pid: u32) -> Result<Vec<Process>, SystemError> {
        let mut chain = Vec::new();
        let mut current_pid = Some(pid);
        let mut loop_detector = std::collections::HashSet::new();

        while let Some(pid) = current_pid {
            if !loop_detector.insert(pid) {
                break;
            }

            match self.sys.get_process_by_pid(pid) {
                Ok(p) => {
                    current_pid = p.parent_pid;
                    chain.push(p);
                }
                Err(_) => break,
            }
        }

        Ok(crate::core::ancestry::build_ancestry_tree(chain))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ports::MockSystemProvider;

    #[test]
    fn test_inspect_pid_found() {
        let mut mock = MockSystemProvider::new();
        mock.expect_get_process_by_pid()
            .with(mockall::predicate::eq(123))
            .times(1)
            .returning(|_| {
                Ok(Process {
                    pid: 123,
                    parent_pid: Some(1),
                    name: "test".to_string(),
                    cmd: vec!["test".to_string()],
                    exe_path: None,
                    uid: None,
                    username: None,
                    start_time: 0,
                    cwd: None,
                    git_repo: None,
                    git_branch: None,
                    container: None,
                    service: None,
                    service_file: None,
                    ports: vec![],
                    bind_addrs: vec![],
                    port_states: vec![],
                    restart_count: None,
                    health: "healthy".into(),
                    forked: "unknown".into(),
                    env: vec![],
                })
            });

        let service = WitrService::new(mock);
        let result = service.inspect_pid(123);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().pid, 123);
    }

    #[test]
    fn test_get_ancestry_chain() {
        let mut mock = MockSystemProvider::new();

        // Target process 100 -> Parent 50 -> Root 1
        mock.expect_get_process_by_pid()
            .with(mockall::predicate::eq(100))
            .returning(|_| {
                Ok(Process {
                    pid: 100,
                    parent_pid: Some(50),
                    name: "target".into(),
                    cmd: vec![],
                    exe_path: None,
                    uid: None,
                    username: None,
                    start_time: 0,
                    cwd: None,
                    git_repo: None,
                    git_branch: None,
                    container: None,
                    service: None,
                    service_file: None,
                    ports: vec![],
                    bind_addrs: vec![],
                    port_states: vec![],
                    restart_count: None,
                    health: "healthy".into(),
                    forked: "unknown".into(),
                    env: vec![],
                })
            });

        mock.expect_get_process_by_pid()
            .with(mockall::predicate::eq(50))
            .returning(|_| {
                Ok(Process {
                    pid: 50,
                    parent_pid: Some(1),
                    name: "parent".into(),
                    cmd: vec![],
                    exe_path: None,
                    uid: None,
                    username: None,
                    start_time: 0,
                    cwd: None,
                    git_repo: None,
                    git_branch: None,
                    container: None,
                    service: None,
                    service_file: None,
                    ports: vec![],
                    bind_addrs: vec![],
                    port_states: vec![],
                    restart_count: None,
                    health: "healthy".into(),
                    forked: "unknown".into(),
                    env: vec![],
                })
            });

        mock.expect_get_process_by_pid()
            .with(mockall::predicate::eq(1))
            .returning(|_| {
                Ok(Process {
                    pid: 1,
                    parent_pid: None,
                    name: "init".into(),
                    cmd: vec![],
                    exe_path: None,
                    uid: None,
                    username: None,
                    start_time: 0,
                    cwd: None,
                    git_repo: None,
                    git_branch: None,
                    container: None,
                    service: None,
                    service_file: None,
                    ports: vec![],
                    bind_addrs: vec![],
                    port_states: vec![],
                    restart_count: None,
                    health: "healthy".into(),
                    forked: "unknown".into(),
                    env: vec![],
                })
            });

        let service = WitrService::new(mock);
        let chain = service.get_ancestry(100).unwrap();

        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0].name, "init"); // Root
        assert_eq!(chain[1].name, "parent");
        assert_eq!(chain[2].name, "target"); // Target
    }

    #[test]
    fn test_inspect_pid_not_found() {
        let mut mock = MockSystemProvider::new();
        mock.expect_get_process_by_pid()
            .with(mockall::predicate::eq(999))
            .times(1)
            .returning(|_| Err(SystemError::ProcessNotFound("999".to_string())));

        let service = WitrService::new(mock);
        let result = service.inspect_pid(999);
        assert!(result.is_err());
    }
}
