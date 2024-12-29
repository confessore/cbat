use serde_derive::Deserialize;

use crate::{client::Client, price_book::PriceBook};

#[derive(Debug, Deserialize)]
pub struct ProductBook {
    pub pricebook: PriceBook,
    pub last: Option<String>,
    pub mid_market: Option<String>,
    pub spread_bps: Option<String>,
    pub spread_absolute: Option<String>,
}

impl ProductBook {
    pub async fn get_public_product_book(
        client: &Client,
        product_id: &str,
        limit: Option<i32>,
        aggregation_price_increment: Option<&str>,
    ) -> Result<ProductBook, reqwest::Error> {
        let limit = match limit {
            Some(limit) => &format!("&limit={}", limit),
            None => "",
        };
        let aggregation_price_increment = match aggregation_price_increment {
            Some(aggregation_price_increment) => &format!(
                "&aggregation_price_increment={}",
                aggregation_price_increment
            ),
            None => "",
        };
        let url = &format!(
            "{}?product_id={}{}{}",
            PUBLIC_PRODUCT_BOOK_URL, product_id, limit, aggregation_price_increment
        );
        let response = client.get(url).await?;
        let product: ProductBook = response.json().await?;
        Ok(product)
    }
}

const PUBLIC_PRODUCT_BOOK_URL: &str =
    "https://api.coinbase.com/api/v3/brokerage/market/product_book";
