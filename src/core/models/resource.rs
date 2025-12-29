use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContext {
    pub energy_impact: String,
    pub prevents_sleep: bool,
    pub thermal_state: String,
    pub app_napped: bool,
}
