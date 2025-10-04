use serde::{Deserialize, Serialize};

use crate::trade::Trade;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketTrades {
    pub trades: Option<Vec<Trade>>,
    pub best_bid: Option<String>,
    pub best_ask: Option<String>,
}
