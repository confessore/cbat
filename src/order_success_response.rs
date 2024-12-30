use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSuccessResponse {
    pub order_id: String,
    pub product_id: Option<String>,
    pub side: Option<String>,
    pub client_order_id: Option<String>,
}
