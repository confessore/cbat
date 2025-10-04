use crate::balance::Balance;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub uuid: String,                        // Unique identifier for account
    pub name: Option<String>,                // Name for the account
    pub currency: Option<String>,            // Currency symbol for the account
    pub available_balance: Balance,          // Available balance in the account
    pub default: Option<bool>, // Whether or not this account is the user's primary account
    pub active: Option<bool>,  // Whether or not this account is active and okay to use
    pub created_at: Option<DateTime<Utc>>, // RFC3339 Timestamp - Time at which this account was created
    pub updated_at: Option<DateTime<Utc>>, // RFC3339 Timestamp - Time at which this account was updated
    pub deleted_at: Option<DateTime<Utc>>, // RFC3339 Timestamp - Time at which this account was deleted
    pub account_type: Option<String>, // Possible values: [ACCOUNT_TYPE_UNSPECIFIED, ACCOUNT_TYPE_CRYPTO, ACCOUNT_TYPE_FIAT, ACCOUNT_TYPE_VAULT, ACCOUNT_TYPE_PERP_FUTURES]
    pub ready: Option<bool>,          // Whether or not this account is ready to trade
    pub hold: Balance, // Amount that is being held for pending transfers against the available balance
    pub retail_portfolio_id: Option<String>, // The ID of the portfolio this account is associated with
    pub platform: Option<String>, // Possible values: [ACCOUNT_PLATFORM_UNSPECIFIED, ACCOUNT_PLATFORM_CONSUMER, ACCOUNT_PLATFORM_CFM_CONSUMER, ACCOUNT_PLATFORM_INTX]
}
