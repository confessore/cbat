use serde::{Deserialize, Serialize};

use crate::edit_order_error::EditOrderError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditOrder {
    pub success: bool,
    pub errors: Vec<EditOrderError>,
}
