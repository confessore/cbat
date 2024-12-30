use serde::{ Deserialize, Serialize };

use crate::{
    order_error_response::OrderErrorResponse,
    order_success_response::OrderSuccessResponse,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub success: bool,
    pub success_response: OrderSuccessResponse,
    pub error_response: Option<OrderErrorResponse>,
    pub message: Option<String>,
}
