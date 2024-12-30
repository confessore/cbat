use serde_derive::Deserialize;

use crate::trade::Trade;

#[derive(Debug, Deserialize)]
pub struct MarketTrades {
    pub trades: Option<Vec<Trade>>,
    pub best_bid: Option<String>,
    pub best_ask: Option<String>,
}
