use serde_derive::Deserialize;

use crate::candle::Candle;

#[derive(Debug, Deserialize)]
pub struct ProductCandles {
    pub candles: Option<Vec<Candle>>,
}
