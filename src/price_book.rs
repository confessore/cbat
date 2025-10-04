use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bid_ask::BidAsk;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceBook {
    pub product_id: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
    pub time: Option<DateTime<Utc>>,
}
