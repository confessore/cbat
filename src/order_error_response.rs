use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    pub error_details: Option<String>,
    pub preview_failure_reason: Option<String>,
    pub new_order_failure_reason: Option<String>,
}
