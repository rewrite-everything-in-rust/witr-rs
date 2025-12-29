use crate::core::models::ResourceContext;

#[cfg(target_os = "macos")]
pub fn get_resource_context(pid: u32) -> Option<ResourceContext> {
    use std::process::Command;
    
    let mut prevents_sleep = false;
    let mut thermal_state = String::new();
    let mut energy_impact = String::new();
    let app_napped = false;
    
    if let Ok(output) = Command::new("pmset").args(["-g", "assertions"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains(&format!("pid {}", pid)) {
            prevents_sleep = true;
        }
    }
    
    if let Ok(output) = Command::new("pmset").args(["-g", "therm"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("Normal") {
            thermal_state = "Normal".to_string();
        } else if output_str.contains("Heavy") {
            thermal_state = "Heavy".to_string();
        }
    }
    
    Some(ResourceContext {
        energy_impact,
        prevents_sleep,
        thermal_state,
        app_napped,
    })
}

#[cfg(not(target_os = "macos"))]
pub fn get_resource_context(_pid: u32) -> Option<ResourceContext> {
    None
}
