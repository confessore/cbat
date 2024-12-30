use serde_derive::Deserialize;

use crate::product::Product;

#[derive(Debug, Deserialize)]
pub struct Products {
    pub products: Option<Vec<Product>>,
}
