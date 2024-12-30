use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitLimitGtc {
    pub quote_size: Option<String>,
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub post_only: Option<bool>,
}
