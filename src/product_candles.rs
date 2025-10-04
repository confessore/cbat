use serde::{Deserialize, Serialize};

use crate::candle::Candle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductCandles {
    pub candles: Option<Vec<Candle>>,
}
