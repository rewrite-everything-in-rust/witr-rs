use std::process::Command;

pub fn get_service_name(pid: u32) -> Option<String> {
    let output = Command::new("tasklist")
        .args([
            "/SVC",
            "/FI",
            &format!("PID eq {}", pid),
            "/FO",
            "CSV",
            "/NH",
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();

    // Output format: "Image Name","PID","Services"
    if line.is_empty() {
        return None;
    }

    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return None;
    }

    let service_part = parts[2].trim_matches('"');

    if service_part == "N/A" {
        None
    } else {
        Some(service_part.to_string())
    }
}

pub fn get_service_start_type(service_name: &str) -> Option<String> {
    let output = Command::new("sc")
        .args(["qc", service_name])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Look for: START_TYPE : 2 AUTO_START
    for line in stdout.lines() {
        if line.trim().starts_with("START_TYPE") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                return Some(parts[3].to_string());
            }
        }
    }
    None
}

pub fn get_service_binary_path(service_name: &str) -> Option<String> {
    let output = Command::new("sc")
        .args(["qc", service_name])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Look for: BINARY_PATH_NAME : path
    for line in stdout.lines() {
        if line.trim().starts_with("BINARY_PATH_NAME") {
            if let Some(idx) = line.find(':') {
                return Some(line[idx + 1..].trim().to_string());
            }
        }
    }
    None
}
