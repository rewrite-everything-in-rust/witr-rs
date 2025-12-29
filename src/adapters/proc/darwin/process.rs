use std::process::Command;

pub struct ProcessInfo {
    pub ppid: u32,
    pub name: String,
    pub uid: u32,
    pub start_time: u64,
}

pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    // ps -p PID -o ppid=,user=,lstart=,comm=
    // Note: 'user' returns username, we might need 'uid'
    // Let's use 'uid' for ps
    if let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "ppid=,uid=,lstart=,comm="])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_ps_info(&output_str);
    }
    None
}

fn parse_ps_info(output: &str) -> Option<ProcessInfo> {
    let line = output.lines().next()?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 7 {
        // PPID + UID + Day + Month + Date + Time + Year + Comm (min)
        return None;
    }

    let ppid = parts[0].parse::<u32>().ok()?;
    let uid = parts[1].parse::<u32>().ok()?;

    let name = parts.last()?.to_string();
    let start_time = 0;

    Some(ProcessInfo {
        ppid,
        name,
        uid,
        start_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ps_info() {
        // Mock output: PPID UID START... COMM
        let output = "  1   501 Mon Jan 16 10:00:00 2023 /sbin/launchd";
        let info = parse_ps_info(output).unwrap();

        assert_eq!(info.ppid, 1);
        assert_eq!(info.uid, 501);
        assert_eq!(info.name, "/sbin/launchd");
    }
}
