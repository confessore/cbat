use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub value: String,    // Amount of currency that this object represents
    pub currency: String, // Denomination of the currency
}
