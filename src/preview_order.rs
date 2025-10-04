use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewOrder {
    pub order_total: String,
    pub commission_total: String,
    pub errs: Vec<String>,
    pub warning: Vec<String>,
    pub quote_size: String,
    pub base_size: String,
    pub best_bid: String,
    pub best_ask: String,
    pub is_max: bool,
    pub order_margin_total: Option<String>,
    pub leverage: Option<String>,
    pub long_leverage: Option<String>,
    pub short_leverage: Option<String>,
    pub slippage: Option<String>,
    pub preview_id: Option<String>,
    pub current_liquidation_buffer: Option<String>,
    pub projected_liquidation_buffer: Option<String>,
    pub max_leverage: Option<String>,
}
