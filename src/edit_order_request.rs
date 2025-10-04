use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditOrderRequest<'a> {
    pub order_id: &'a str,
    pub price: String,
    pub size: String,
}
