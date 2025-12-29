use std::process::Command;

pub fn get_resource_context(_pid: u32) -> Option<String> {
    let assertions_out = Command::new("pmset")
        .args(["-g", "assertions"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .ok();

    let thermlog_out = Command::new("pmset")
        .args(["-g", "thermlog"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .ok();

    parse_resource_context(assertions_out.as_deref(), thermlog_out.as_deref())
}

fn parse_resource_context(assertions: Option<&str>, thermlog: Option<&str>) -> Option<String> {
    let mut prevents_sleep = false;
    if let Some(assert_str) = assertions {
        if assert_str.contains("PreventUserIdleSystemSleep") {
            prevents_sleep = true;
        }
    }

    let thermal_state = if let Some(therm_str) = thermlog {
        therm_str.lines().next().unwrap_or("").to_string()
    } else {
        String::new()
    };

    if prevents_sleep || !thermal_state.is_empty() {
        Some(format!("sleep={}, thermal={}", prevents_sleep, thermal_state))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resource_context() {
        let assertions = "Some output... PreventUserIdleSystemSleep ...";
        let thermlog = "Normal";
        
        // Both present
        let res = parse_resource_context(Some(assertions), Some(thermlog));
        assert_eq!(res, Some("sleep=true, thermal=Normal".to_string()));
        
        // Sleep only
        let res2 = parse_resource_context(Some(assertions), None);
        assert_eq!(res2, Some("sleep=true, thermal=".to_string()));
        
        // None
        let res3 = parse_resource_context(Some("nada"), Some(""));
        assert_eq!(res3, None); // thermal empty, sleep false
    }
}
