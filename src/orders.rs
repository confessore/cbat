use serde::{Deserialize, Serialize};

use crate::order::Order;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orders {
    pub orders: Option<Vec<Order>>,
    pub order: Option<Order>,
}
