use serde_derive::Deserialize;

use crate::{client::Client, trade::Trade};

#[derive(Debug, Deserialize)]
pub struct MarketTrades {
    pub trades: Option<Vec<Trade>>,
    pub best_bid: Option<String>,
    pub best_ask: Option<String>,
}

impl MarketTrades {
    pub async fn get_public_market_trades(
        client: &Client<'_>,
        product_id: &str,
        limit: u32,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<MarketTrades, reqwest::Error> {
        let start = match start {
            Some(start) => &format!("&start={}", start),
            None => "",
        };
        let end = match end {
            Some(end) => &format!("&end={}", end),
            None => "",
        };
        let url = &format!(
            "{}{}/ticker?limit={}{}{}",
            PUBLIC_MARKET_TRADES_URL, product_id, limit, start, end
        );
        let response = client.get(url).await?;
        let market_trades: MarketTrades = response.json().await?;
        Ok(market_trades)
    }
}

const PUBLIC_MARKET_TRADES_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products/";
