use serde::{ Deserialize, Serialize };

use crate::fill::Fill;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fills {
    pub fills: Option<Vec<Fill>>,
}
