use serde_derive::Deserialize;

use crate::{
    client::{create_jwt, Client},
    trade::Trade,
};

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
        let response = client
            .get_auth(url, &create_jwt("GET", PUBLIC_MARKET_TRADES_ENDPOINT))
            .await?;
        let market_trades: MarketTrades = response.json().await?;
        Ok(market_trades)
    }
}

const PUBLIC_MARKET_TRADES_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products/";
const PUBLIC_MARKET_TRADES_ENDPOINT: &str = "/api/v3/brokerage/market/products/ticker";
