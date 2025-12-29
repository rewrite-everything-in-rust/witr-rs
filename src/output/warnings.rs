use crate::core::models::Process;

pub fn print(chain: &[Process]) {
    for p in chain {
        if p.health != "healthy" {
            println!("⚠  PID {} is {}", p.pid, p.health);
        }
        if p.uid == Some("0".to_string()) {
            println!("⚠  PID {} is running as root", p.pid);
        }
        if !p.ports.is_empty() {
            for (port, addr) in p.ports.iter().zip(&p.bind_addrs) {
                if addr.starts_with("0.0.0.0") || addr == "::" {
                    println!(
                        "⚠  PID {} is listening publicly on {}:{}",
                        p.pid, addr, port
                    );
                }
            }
        }
    }
}
