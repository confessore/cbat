use serde_derive::Deserialize;
use crate::portfolio::Portfolio;

#[derive(Debug, Deserialize)]
pub struct Portfolios {
    pub portfolios: Option<Vec<Portfolio>>,
}
