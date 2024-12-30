use std::fmt::{ Display, Formatter, Result };

#[derive(Debug)]
pub enum PortfolioType {
    Undefined,
    Default,
    Consumer,
    INTX,
}

impl Display for PortfolioType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl PortfolioType {
    pub fn as_str(&self) -> &str {
        match self {
            PortfolioType::Undefined => "UNDEFINED",
            PortfolioType::Default => "DEFAULT",
            PortfolioType::Consumer => "CONSUMER",
            PortfolioType::INTX => "INTX",
        }
    }
}
