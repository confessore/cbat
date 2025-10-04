use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Display for OrderSide {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl OrderSide {
    pub fn as_str(&self) -> &str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}
