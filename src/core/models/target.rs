use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetType {
    Name,
    Pid,
    Port,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub target_type: TargetType,
    pub value: String,
}
