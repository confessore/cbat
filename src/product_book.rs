use serde_derive::Deserialize;

use crate::price_book::PriceBook;

#[derive(Debug, Deserialize)]
pub struct ProductBook {
    pub pricebook: PriceBook,
    pub last: Option<String>,
    pub mid_market: Option<String>,
    pub spread_bps: Option<String>,
    pub spread_absolute: Option<String>,
}
