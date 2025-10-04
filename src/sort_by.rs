use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortBy {
    UnknownSortBy,
    LimitPrice,
    LastFillTime,
}

impl Display for SortBy {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl SortBy {
    pub fn as_str(&self) -> &str {
        match self {
            SortBy::UnknownSortBy => "UNKNOWN_SORT_BY",
            SortBy::LimitPrice => "LIMIT_PRICE",
            SortBy::LastFillTime => "LAST_FILL_TIME",
        }
    }
}
