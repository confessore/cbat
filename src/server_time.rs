use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ServerTime {
    pub iso: Option<String>,
    pub epochSeconds: Option<String>,
    pub epochMillis: Option<String>,
}
