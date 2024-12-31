use serde::{ Deserialize, Serialize };

use crate::order::Order;

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub orders: Vec<Order>,
}
