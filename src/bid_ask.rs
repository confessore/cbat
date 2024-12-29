use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BidAsk {
    pub price: String,
    pub size: String,
}
