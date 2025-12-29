use crate::core::models::Process;
use sysinfo::{Pid, System};

pub fn get_process_info(pid: u32, system: &System) -> Option<Process> {
    let sys_pid = Pid::from_u32(pid);
    if let Some(proc) = system.process(sys_pid) {
        return Some(Process {
            pid,
            parent_pid: proc.parent().map(|p| p.as_u32()),
            name: proc.name().to_string_lossy().to_string(),
            cmd: proc
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            exe_path: proc.exe().map(|p| p.display().to_string()),
            ..Default::default()
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_get_process_info() {
        let mut sys = System::new_all();
        sys.refresh_all();
        let pid = std::process::id();

        let proc = get_process_info(pid, &sys);
        assert!(proc.is_some());
        assert_eq!(proc.unwrap().pid, pid);
    }
}
