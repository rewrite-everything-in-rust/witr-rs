use crate::core::models::Process;

pub fn print(chain: &[Process], _indent: usize) {
    for (i, p) in chain.iter().enumerate() {
        if i == 0 {
            println!("{} (pid {})", p.name, p.pid);
        } else {
            let prefix = "  ".repeat(i - 1);
            println!("{}└─ {} (pid {})", prefix, p.name, p.pid);
        }
    }
}
