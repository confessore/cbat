use chrono::{DateTime, Local};
use serde_derive::Deserialize;

use crate::bid_ask::BidAsk;

#[derive(Debug, Deserialize)]
pub struct PriceBook {
    pub product_id: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
    pub time: Option<DateTime<Local>>,
}
