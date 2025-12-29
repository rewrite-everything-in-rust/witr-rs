use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContext {
    pub open_files: u32,
    pub file_limit: u32,
    pub locked_files: Vec<String>,
    pub watched_dirs: Vec<String>,
}
