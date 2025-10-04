use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerTime {
    pub iso: Option<String>,
    pub epochSeconds: Option<String>,
    pub epochMillis: Option<String>,
}
