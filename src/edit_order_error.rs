use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct EditOrderError {
    pub edit_failure_reason: Option<String>,
    pub preview_faulure_reason: Option<String>,
}
