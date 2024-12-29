use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub start: Option<String>,
    pub low: Option<String>,
    pub high: Option<String>,
    pub open: Option<String>,
    pub close: Option<String>,
    pub volume: Option<String>,
}
