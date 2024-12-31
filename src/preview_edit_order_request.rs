use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewEditOrderRequest<'a> {
    pub order_id: &'a str,
    pub price: String,
    pub size: String,
}
