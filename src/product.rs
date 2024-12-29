use serde_derive::Deserialize;

use crate::{
    client::{create_jwt, Client},
    fcm_trading_session_details::FCMTradingSessionDetails,
    future_product_details::FutureProductDetails,
};

#[derive(Debug, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub price: String,
    pub price_percentage_change_24h: String,
    pub volume_24h: String,
    pub volume_percentage_change_24h: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub quote_min_size: String,
    pub quote_max_size: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub base_name: String,
    pub quote_name: String,
    pub watched: bool,
    pub is_disabled: bool,
    pub new: bool,
    pub status: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
    pub auction_mode: bool,
    pub product_type: Option<String>,
    pub quote_currency_id: Option<String>,
    pub base_currency_id: Option<String>,
    pub fcm_trading_session_details: Option<FCMTradingSessionDetails>,
    pub mid_market_price: Option<String>,
    pub alias: Option<String>,
    pub alias_to: Option<Vec<String>>,
    pub base_display_symbol: String,
    pub quote_display_symbol: String,
    pub view_only: Option<bool>,
    pub price_increment: Option<String>,
    pub display_name: Option<String>,
    pub product_venue: Option<String>,
    pub approximate_quote_24h_volume: Option<String>,
    pub future_product_details: Option<FutureProductDetails>,
    pub contract_display_name: Option<String>,
    pub time_to_expiry_ms: Option<u64>,
    pub non_crypto: Option<bool>,
    pub contract_expiry_name: Option<String>,
    pub twenty_four_by_seven: Option<bool>,
}

impl Product {
    pub async fn get_public_product(
        client: &Client<'_>,
        product_id: &str,
    ) -> Result<Product, reqwest::Error> {
        let url = &format!("{}/{}", PUBLIC_PRODUCT_URL, product_id);
        let response = client
            .get_auth(url, &create_jwt("GET", PUBLIC_PRODUCT_ENDPOINT))
            .await?;
        let product: Product = response.json().await?;
        Ok(product)
    }
}

const PUBLIC_PRODUCT_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
const PUBLIC_PRODUCT_ENDPOINT: &str = "/api/v3/brokerage/market/products";
