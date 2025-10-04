use std::fmt::{Display, Formatter, Result};

pub enum Granularity {
    Unknown,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    TwoHours,
    SixHours,
    OneDay,
}

impl Display for Granularity {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Granularity {
    pub fn as_str(&self) -> &str {
        match self {
            Granularity::Unknown => "UNKNOWN_GRANULARITY",
            Granularity::OneMinute => "ONE_MINUTE",
            Granularity::FiveMinutes => "FIVE_MINUTE",
            Granularity::FifteenMinutes => "FIFTEEN_MINUTE",
            Granularity::OneHour => "ONE_HOUR",
            Granularity::TwoHours => "TWO_HOUR",
            Granularity::SixHours => "SIX_HOUR",
            Granularity::OneDay => "ONE_DAY",
        }
    }
}
