use serde::{ Deserialize, Serialize };

use crate::order_configuration::OrderConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub client_order_id: String,
    pub product_id: String,
    pub side: String,
    pub order_configuration: OrderConfiguration,
    pub leverage: Option<String>,
    pub margin_type: Option<String>,
    pub retail_portfolio_id: Option<String>,
    pub preview_id: Option<String>,
}
