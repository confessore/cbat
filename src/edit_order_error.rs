use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditOrderError {
    pub edit_failure_reason: Option<String>,
    pub preview_faulure_reason: Option<String>,
}
