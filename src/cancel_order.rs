use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CancelOrder {
    pub success: bool,
    pub failure_reason: String,
    pub order_id: String,
}
