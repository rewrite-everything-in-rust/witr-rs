use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Process {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub cmd: Vec<String>,
    pub exe_path: Option<String>,
    pub uid: Option<String>,
    pub username: Option<String>,
    pub start_time: u64,
    pub cwd: Option<String>,
    pub git_repo: Option<String>,
    pub git_branch: Option<String>,
    pub container: Option<String>,
    pub service: Option<String>,
    pub ports: Vec<u16>,
    pub bind_addrs: Vec<String>,
    pub health: String,
    pub forked: String,
    pub env: Vec<String>,
}
