use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Fill {
    pub entry_id: Option<String>,
    pub trade_id: Option<String>,
    pub order_id: Option<String>,
    pub trade_time: Option<DateTime<Utc>>,
    pub trade_type: Option<String>,
    pub price: Option<String>,
    pub size: Option<String>,
    pub commission: Option<String>,
    pub product_id: Option<String>,
    pub sequence_timestamp: Option<DateTime<Utc>>,
    pub liquidity_indicator: Option<String>,
    pub size_in_quote: Option<bool>,
    pub user_id: Option<String>,
    pub side: Option<String>,
    pub retail_portfolio_id: Option<String>,
}
