use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ExpiringContractStatus {
    Unknown,
    Unexpired,
    Expired,
    All
}

impl Display for ExpiringContractStatus {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl ExpiringContractStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ExpiringContractStatus::Unknown => "UNKNOWN_EXPIRING_CONTRACT_STATUS",
            ExpiringContractStatus::Unexpired => "STATUS_UNEXPIRED",
            ExpiringContractStatus::Expired => "STATUS_EXPIRED",
            ExpiringContractStatus::All => "STATUS_ALL",
        }
    }
}