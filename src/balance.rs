use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub value: String, // Amount of currency that this object represents
    pub currency: String, // Denomination of the currency
}
