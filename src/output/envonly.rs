use crate::core::models::Process;

pub fn print(process: &Process) {
    for env_var in &process.env {
        println!("{}", env_var);
    }
}
