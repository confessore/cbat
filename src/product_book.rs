use serde::{Deserialize, Serialize};

use crate::price_book::PriceBook;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductBook {
    pub pricebook: PriceBook,
    pub last: Option<String>,
    pub mid_market: Option<String>,
    pub spread_bps: Option<String>,
    pub spread_absolute: Option<String>,
}
