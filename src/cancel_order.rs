use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelOrder {
    pub success: bool,
    pub failure_reason: String,
    pub order_id: String,
}
