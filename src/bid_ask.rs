use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BidAsk {
    pub price: String,
    pub size: String,
}
