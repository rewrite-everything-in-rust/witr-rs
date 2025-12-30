use sysinfo::{Pid, System};

pub fn get_cmdline(pid: u32, system: &System) -> Vec<String> {
    if let Some(process) = system.process(Pid::from_u32(pid)) {
        return process
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_cmdline_from_sysinfo() {
        let mut sys = System::new_all();
        sys.refresh_all();

        let self_pid = std::process::id();
        let cmd = get_cmdline(self_pid, &sys);

        assert!(!cmd.is_empty());
    }
}
