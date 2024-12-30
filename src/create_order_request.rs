use serde::{ Deserialize, Serialize };

use crate::order_configuration::OrderConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest<'a> {
    pub client_order_id: &'a str,
    pub product_id: &'a str,
    pub side: &'a str,
    pub order_configuration: OrderConfiguration,
    pub leverage: Option<&'a str>,
    pub margin_type: Option<&'a str>,
    pub retail_portfolio_id: Option<&'a str>,
    pub preview_id: Option<&'a str>,
}
