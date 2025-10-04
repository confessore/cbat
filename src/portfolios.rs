use serde::{Deserialize, Serialize};

use crate::portfolio::Portfolio;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Portfolios {
    pub portfolios: Option<Vec<Portfolio>>,
}
