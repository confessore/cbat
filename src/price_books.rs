use serde_derive::Deserialize;

use crate::price_book::PriceBook;

#[derive(Debug, Deserialize)]
pub struct PriceBooks {
    pub pricebooks: Option<Vec<PriceBook>>,
    pub pricebook: Option<PriceBook>,
}
