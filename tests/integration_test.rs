use witr_rs::adapters::system::RealSystem;
use witr_rs::core::service::WitrService;

#[test]
fn test_inspect_self() {
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    let current_pid = std::process::id();
    let result = service.inspect_pid(current_pid);

    assert!(result.is_ok());
    let process = result.unwrap();
    assert_eq!(process.pid, current_pid);
    assert!(!process.name.is_empty());
}

#[test]
fn test_get_ancestry_of_self() {
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    let current_pid = std::process::id();
    let result = service.get_ancestry(current_pid);

    assert!(result.is_ok());
    let chain = result.unwrap();
    assert!(!chain.is_empty());
    assert_eq!(chain.last().unwrap().pid, current_pid);
}

#[test]
fn test_inspect_system_process() {
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    #[cfg(target_os = "windows")]
    let system_pid = 4;

    #[cfg(target_os = "linux")]
    let system_pid = 1;

    #[cfg(target_os = "macos")]
    let system_pid = 1;

    let result = service.inspect_pid(system_pid);
    assert!(result.is_ok());
}

#[test]
fn test_inspect_nonexistent_process() {
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    let result = service.inspect_pid(999999);
    assert!(result.is_err());
}

#[test]
fn test_full_inspection_flow() {
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    let current_pid = std::process::id();

    let inspection = service.get_inspection(current_pid);
    assert!(inspection.is_ok());

    let result = inspection.unwrap();
    assert_eq!(result.process.pid, current_pid);
    assert!(!result.ancestry.is_empty());
    assert!(!result.source.source_type.to_string().is_empty());
}
