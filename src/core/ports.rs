use crate::core::models::Process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Process not found: {0}")]
    ProcessNotFound(String),
    #[error("System error: {0}")]
    Unknown(String),
}

#[cfg_attr(test, mockall::automock)]
pub trait SystemProvider {
    fn get_process_by_pid(&self, pid: u32) -> Result<Process, SystemError>;
    fn find_processes_by_name(&self, name: &str) -> Result<Vec<Process>, SystemError>;
    fn find_process_by_port(&self, port: u16) -> Result<Process, SystemError>;
}
