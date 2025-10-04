use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewEditOrderError {
    pub edit_failure_reason: Option<String>,
    pub preview_failure_reason: Option<String>,
}
