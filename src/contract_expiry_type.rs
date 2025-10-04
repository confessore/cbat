use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ContractExpiryType {
    Unknown,
    Expiring,
    Perpetual,
}

impl Display for ContractExpiryType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl ContractExpiryType {
    pub fn as_str(&self) -> &str {
        match self {
            ContractExpiryType::Unknown => "UNKNOWN_CONTRACT_EXPIRY_TYPE",
            ContractExpiryType::Expiring => "EXPIRING",
            ContractExpiryType::Perpetual => "PERPETUAL",
        }
    }
}
