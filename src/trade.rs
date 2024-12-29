use chrono::{DateTime, Local};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub trade_id: Option<String>,
    pub product_id: Option<String>,
    pub price: Option<String>,
    pub size: Option<String>,
    pub time: Option<DateTime<Local>>,
    pub side: Option<String>,
    pub exchange: Option<String>,
}
