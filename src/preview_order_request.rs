use serde::{ Deserialize, Serialize };

use crate::order_configuration::OrderConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewOrderRequest<'a> {
    pub product_id: &'a str,
    pub side: &'a str,
    pub order_configuration: OrderConfiguration,
    pub leverage: Option<&'a str>,
    pub margin_type: Option<&'a str>,
    pub retail_portfolio_id: Option<&'a str>,
}
