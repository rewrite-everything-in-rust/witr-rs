use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub source_type: SourceType,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    System,
    Systemd,
    Launchd,
    Docker,
    Manual,
    PM2,
    Supervisor,
    Cron,
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SourceType::System => "system",
            SourceType::Systemd => "systemd",
            SourceType::Launchd => "launchd",
            SourceType::Docker => "docker",
            SourceType::Manual => "manual",
            SourceType::PM2 => "pm2",
            SourceType::Supervisor => "supervisor",
            SourceType::Cron => "cron",
        };
        write!(f, "{}", s)
    }
}
