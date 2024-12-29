use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ProductType {
    Unknown,
    Spot,
    Future,
}

impl Display for ProductType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl ProductType {
    pub fn as_str(&self) -> &str {
        match self {
            ProductType::Unknown => "UNKNOWN_PRODUCT_TYPE",
            ProductType::Spot => "SPOT",
            ProductType::Future => "FUTURE",
        }
    }
}