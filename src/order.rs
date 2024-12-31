use serde::{ Deserialize, Serialize };

use crate::{ order_configuration::OrderConfiguration, prelude::EditHistory };

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub product_id: String,
    pub user_id: String,
    pub order_configuration: OrderConfiguration,
    pub side: String, // Possible values: [BUY, SELL]
    pub client_order_id: String,
    pub status: String, // Possible values: [PENDING, OPEN, FILLED, CANCELLED, EXPIRED, FAILED, UNKNOWN_ORDER_STATUS, QUEUED, CANCEL_QUEUED]
    pub time_in_force: Option<String>, // Possible values: [UNKNOWN_TIME_IN_FORCE, GOOD_UNTIL_DATE_TIME, GOOD_UNTIL_CANCELLED, IMMEDIATE_OR_CANCEL, FILL_OR_KILL]
    pub created_time: String, // RFC3339 Timestamp
    pub completion_percentage: String,
    pub filled_size: Option<String>,
    pub average_filled_price: String,
    pub fee: Option<String>, // Deprecated
    pub number_of_fills: String,
    pub filled_value: Option<String>,
    pub pending_cancel: bool,
    pub size_in_quote: bool,
    pub total_fees: String,
    pub size_inclusive_of_fees: bool,
    pub total_value_after_fees: String,
    pub trigger_status: Option<String>, // Possible values: [UNKNOWN_TRIGGER_STATUS, INVALID_ORDER_TYPE, STOP_PENDING, STOP_TRIGGERED]
    pub order_type: Option<String>, // Possible values: [UNKNOWN_ORDER_TYPE, MARKET, LIMIT, STOP, STOP_LIMIT, BRACKET]
    pub reject_reason: Option<String>, // Possible values: [REJECT_REASON_UNSPECIFIED, HOLD_FAILURE, TOO_MANY_OPEN_ORDERS, REJECT_REASON_INSUFFICIENT_FUNDS, RATE_LIMIT_EXCEEDED]
    pub settled: bool,
    pub product_type: Option<String>, // Possible values: [UNKNOWN_PRODUCT_TYPE, SPOT, FUTURE]
    pub reject_message: Option<String>,
    pub cancel_message: Option<String>,
    pub order_placement_source: Option<String>, // Possible values: [UNKNOWN_PLACEMENT_SOURCE, RETAIL_SIMPLE, RETAIL_ADVANCED]
    pub outstanding_hold_amount: Option<String>,
    pub is_liquidation: bool,
    pub last_fill_time: Option<String>, // RFC3339 Timestamp
    pub edit_history: Option<Vec<EditHistory>>,
    pub price: Option<String>,
    pub size: Option<String>,
    pub replace_accept_timestamp: Option<String>, // RFC3339 Timestamp
    pub leverage: Option<String>,
    pub margin_type: Option<String>, // Possible values: [CROSS, ISOLATED]
    pub retail_portfolio_id: Option<String>,
    pub sequence: Option<i64>, // Deprecated
    pub has_next: bool,
    pub cursor: Option<String>,
}
