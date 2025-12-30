use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SocketInfo {
    pub port: u16,
    pub state: String,
    pub local_addr: String,
    pub remote_addr: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub explanation: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub workaround: String,
}

impl SocketInfo {
    pub fn new(port: u16, state: String, local_addr: String, remote_addr: String) -> Self {
        let mut socket = Self {
            port,
            state: state.clone(),
            local_addr,
            remote_addr,
            explanation: String::new(),
            workaround: String::new(),
        };
        socket.enrich_details();
        socket
    }

    pub fn enrich_details(&mut self) {
        match self.state.as_str() {
            "LISTEN" | "LISTENING" => {
                self.explanation = "Actively listening for connections".to_string();
            }
            "TIME_WAIT" => {
                self.explanation = "Connection closed, waiting for delayed packets".to_string();
                self.workaround = "Wait for timeout (usually 60s) or use SO_REUSEADDR".to_string();
            }
            "CLOSE_WAIT" => {
                self.explanation =
                    "Remote side closed connection, local side has not closed yet".to_string();
                self.workaround = "The application should call close() on the socket".to_string();
            }
            "FIN_WAIT_1" => {
                self.explanation =
                    "Local side initiated close, waiting for acknowledgment".to_string();
            }
            "FIN_WAIT_2" => {
                self.explanation = "Local close acknowledged, waiting for remote close".to_string();
            }
            "ESTABLISHED" => {
                self.explanation = "Active connection".to_string();
            }
            "SYN_SENT" => {
                self.explanation = "Connection request sent, waiting for response".to_string();
            }
            "SYN_RECV" | "SYN_RECEIVED" => {
                self.explanation =
                    "Connection request received, sending acknowledgment".to_string();
            }
            "CLOSING" => {
                self.explanation = "Both sides initiated close simultaneously".to_string();
            }
            "LAST_ACK" => {
                self.explanation = "Waiting for final acknowledgment of close".to_string();
            }
            _ => {
                self.explanation = format!("Socket in {} state", self.state);
            }
        }
    }

    pub fn is_problematic(&self) -> bool {
        matches!(
            self.state.as_str(),
            "TIME_WAIT" | "CLOSE_WAIT" | "FIN_WAIT_1" | "FIN_WAIT_2"
        )
    }
}
