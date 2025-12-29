use sysinfo::ProcessStatus;

pub fn get_health_status(
    _pid: u32,
    status: ProcessStatus,
    memory: u64,
    cpu_usage: f32,
    start_time: u64,
) -> String {
    if matches!(status, ProcessStatus::Zombie) {
        return "zombie".to_string();
    }
    if matches!(status, ProcessStatus::Stop) {
        return "stopped".to_string();
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let uptime = now.saturating_sub(start_time);

    let memory_mb = memory / 1024 / 1024;
    if memory_mb > 1000 {
        return "high-mem".to_string();
    }
    if cpu_usage > 80.0 {
        return "high-cpu".to_string();
    }
    if uptime > 90 * 24 * 3600 {
        return "long-running".to_string();
    }

    "healthy".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_health_status() {
        // Mock params
        let mem = 0; // 0 MB
        let cpu = 0.0;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let recent_start = now - 1000; // Started 1000s ago

        assert_eq!(get_health_status(1, ProcessStatus::Zombie, mem, cpu, recent_start), "zombie");
        assert_eq!(get_health_status(1, ProcessStatus::Stop, mem, cpu, recent_start), "stopped");
        
        // High CPU > 80.0
        assert_eq!(get_health_status(1, ProcessStatus::Run, mem, 85.0, recent_start), "high-cpu");
        
        // Healthy
        assert_eq!(get_health_status(1, ProcessStatus::Run, mem, 10.0, recent_start), "healthy");
    }
}

pub fn detect_forked(parent_pid: Option<u32>) -> String {
    if parent_pid == Some(1) || parent_pid.is_none() {
        "no".to_string()
    } else {
        "forked".to_string()
    }
}
