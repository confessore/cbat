use serde::{Deserialize, Serialize};

use crate::cancel_order::CancelOrder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelOrders {
    pub results: Option<Vec<CancelOrder>>,
}
