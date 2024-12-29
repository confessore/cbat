use serde_derive::Deserialize;

use crate::{
    candle::Candle,
    client::Client,
    granularity::{self, Granularity},
};

#[derive(Debug, Deserialize)]
pub struct ProductCandles {
    pub candles: Option<Vec<Candle>>,
}

impl ProductCandles {
    pub async fn get_public_product_candles(
        client: &Client<'_>,
        product_id: &str,
        start: &str,
        end: &str,
        granularity: Granularity,
        limit: Option<u32>,
    ) -> Result<ProductCandles, reqwest::Error> {
        let limit = match limit {
            Some(limit) => &format!("&limit={}", limit),
            None => "",
        };

        let url = &format!(
            "{}/{}/candles?start={}&end={}&granularity={}{}",
            PUBLIC_PRODUCT_URL, product_id, start, end, granularity, limit
        );
        let response = client.get(url).await?;
        let product_candles: ProductCandles = response.json().await?;
        Ok(product_candles)
    }
}

const PUBLIC_PRODUCT_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
