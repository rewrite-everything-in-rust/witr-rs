use std::fs;

pub fn get_open_files(_pid: u32) -> Vec<String> {
    Vec::new()
}

pub fn get_file_limit(pid: u32) -> Option<(u64, u64)> {
    let limits_path = format!("/proc/{}/limits", pid);
    if let Ok(content) = fs::read_to_string(&limits_path) {
        return parse_file_limit(&content);
    }
    None
}

fn parse_file_limit(content: &str) -> Option<(u64, u64)> {
    for line in content.lines() {
        if line.contains("Max open files") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                if let (Ok(soft), Ok(_hard)) = (parts[3].parse::<u64>(), parts[4].parse::<u64>()) {
                    let current = 0; // Placeholder
                    return Some((current, soft));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_limit() {
        let content =
            "Limit                     Soft Limit           Hard Limit           Units     
Max cpu time              unlimited            unlimited            seconds   
Max file size             unlimited            unlimited            bytes     
Max data size             unlimited            unlimited            bytes     
Max stack size            8388608              unlimited            bytes     
Max core file size        0                    unlimited            bytes     
Max resident set          unlimited            unlimited            bytes     
Max processes             62382                62382                processes 
Max open files            1024                 1048576              files     
Max locked memory         65536                65536                bytes     
Max address space         unlimited            unlimited            bytes     
Max file locks            unlimited            unlimited            locks     
Max pending signals       62382                62382                signals   
Max msgqueue size         819200               819200               bytes     
Max nice priority         0                    0                    
Max realtime priority     0                    0                    
Max realtime timeout      unlimited            unlimited            us        
";
        let result = parse_file_limit(content);
        assert!(result.is_some());
        let (current, limit) = result.unwrap();
        assert_eq!(current, 0);
        assert_eq!(limit, 1024);
    }
}
