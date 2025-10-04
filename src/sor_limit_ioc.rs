use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SorLimitIoc {
    pub quote_size: Option<String>,
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
}
