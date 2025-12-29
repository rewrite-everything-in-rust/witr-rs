use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketInfo {
    pub port: u16,
    pub state: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub explanation: String,
    pub workaround: String,
}
