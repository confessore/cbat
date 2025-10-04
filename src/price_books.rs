use serde::{Deserialize, Serialize};

use crate::price_book::PriceBook;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceBooks {
    pub pricebooks: Option<Vec<PriceBook>>,
    pub pricebook: Option<PriceBook>,
}
