pub fn is_supervisor_process(_pid: u32) -> bool {
    // Stub
    false
}

#[allow(dead_code)]
fn parse_supervisor_conf(content: &str) -> Vec<String> {
    let mut programs = Vec::new();
    for line in content.lines() {
        if line.starts_with("[program:") {
            let name = line.trim_start_matches("[program:").trim_end_matches(']');
            programs.push(name.to_string());
        }
    }
    programs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_supervisor_conf() {
        let conf = "[program:app]\ncommand=/bin/app\n[program:worker]\ncommand=...";
        let progs = parse_supervisor_conf(conf);
        assert_eq!(progs, vec!["app", "worker"]);
    }
}
