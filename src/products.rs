use serde::{Deserialize, Serialize};

use crate::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Products {
    pub products: Option<Vec<Product>>,
}
