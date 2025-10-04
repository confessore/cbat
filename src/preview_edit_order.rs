use serde::{Deserialize, Serialize};

use crate::preview_edit_order_error::PreviewEditOrderError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewEditOrder {
    pub errors: Vec<PreviewEditOrderError>,
    pub slippage: Option<String>,
    pub order_total: Option<String>,
    pub commission_total: Option<String>,
    pub quote_size: Option<String>,
    pub base_size: Option<String>,
    pub best_bid: Option<String>,
    pub best_ask: Option<String>,
    pub average_filled_price: Option<String>,
}
