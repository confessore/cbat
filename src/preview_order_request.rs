use serde::{Deserialize, Serialize};

use crate::order_configuration::OrderConfiguration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewOrderRequest<'a> {
    pub product_id: &'a str,
    pub side: &'a str,
    pub order_configuration: OrderConfiguration,
    pub leverage: Option<String>,
    pub margin_type: Option<String>,
    pub retail_portfolio_id: Option<String>,
}
