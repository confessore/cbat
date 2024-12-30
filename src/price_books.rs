use serde_derive::Deserialize;

use crate::price_book::PriceBook;

#[derive(Debug, Deserialize)]
pub struct PriceBooks {
    pub pricebooks: Vec<PriceBook>,
}
