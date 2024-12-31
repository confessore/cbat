use chrono::{ DateTime, Utc };
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PerpetualDetails {
    pub open_interest: Option<String>,
    pub funding_rate: Option<String>,
    pub funding_time: Option<DateTime<Utc>>,
    pub max_leverage: Option<String>,
    pub base_asset_uuid: Option<String>,
    pub underlying_type: Option<String>,
}
