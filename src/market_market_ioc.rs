use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketMarketIoc {
    pub quote_size: Option<String>,
    pub base_size: Option<String>,
}
