use serde_derive::Deserialize;

use crate::cancel_order::CancelOrder;

#[derive(Debug, Deserialize)]
pub struct CancelOrders {
    pub results: Option<Vec<CancelOrder>>,
}
