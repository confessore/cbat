use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewEditOrderRequest<'a> {
    pub order_id: &'a str,
    pub price: &'a str,
    pub size: &'a str,
}
