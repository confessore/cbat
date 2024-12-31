use chrono::{ DateTime, Utc };
use serde_derive::Deserialize;

use crate::perpetual_details::PerpetualDetails;

#[derive(Debug, Deserialize)]
pub struct FutureProductDetails {
    pub venue: Option<String>,
    pub contract_code: Option<String>,
    pub contract_expiry: Option<DateTime<Utc>>,
    pub contract_size: Option<String>,
    pub contract_root_unit: Option<String>,
    pub group_description: Option<String>,
    pub contract_expiry_timezone: Option<String>,
    pub group_short_description: Option<String>,
    pub risk_managed_by: Option<String>,
    pub contract_expiry_type: Option<String>,
    pub perpetual_details: Option<PerpetualDetails>,
}
