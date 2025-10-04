use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum OrderPlacementSource {
    UnknownPlacement,
    RetailSimple,
    RetailAdvanced,
}

impl Display for OrderPlacementSource {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl OrderPlacementSource {
    pub fn as_str(&self) -> &str {
        match self {
            OrderPlacementSource::UnknownPlacement => "UNKNOWN_PLACEMENT",
            OrderPlacementSource::RetailSimple => "RETAIL_SIMPLE",
            OrderPlacementSource::RetailAdvanced => "RETAIL_ADVANCED",
        }
    }
}
