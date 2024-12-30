use crate::{ client::{ create_jwt, Client }, http_method::HttpMethod, price_books::PriceBooks };

pub struct ApiProducts;

impl ApiProducts {
    pub async fn get_best_bid_ask(
        client: &Client<'_>,
        product_ids: Option<Vec<&str>>
    ) -> Result<PriceBooks, reqwest::Error> {
        let mut query_params = Vec::new();
        if let Some(product_ids) = product_ids {
            for product_id in product_ids {
                query_params.push(format!("product_ids={}", product_id));
            }
        }
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}{}", BEST_BID_ASK_URL, query_string);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), BEST_BID_ASK_ENDPOINT)
        ).await?;
        let price_books: PriceBooks = response.json().await?;
        Ok(price_books)
    }

    pub async fn get_product_book(
        client: &Client<'_>,
        product_id: &str,
        limit: Option<u32>,
        aggregation_price_increment: Option<&str>
    ) -> Result<PriceBooks, reqwest::Error> {
        let mut query_params = Vec::new();
        query_params.push(format!("product_id={}", product_id));
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(aggregation_price_increment) = aggregation_price_increment {
            query_params.push(
                format!("aggregation_price_increment={}", aggregation_price_increment)
            );
        }
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}{}", PRODUCT_BOOK_URL, query_string);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), PRODUCT_BOOK_ENDPOINT)
        ).await?;
        let price_books: PriceBooks = response.json().await?;
        Ok(price_books)
    }
}

const BEST_BID_ASK_URL: &str = "https://api.coinbase.com/api/v3/brokerage/best_bid_ask";
const BEST_BID_ASK_ENDPOINT: &str = "/api/v3/brokerage/best_bid_ask";
const PRODUCT_BOOK_URL: &str = "https://api.coinbase.com/api/v3/brokerage/product_book";
const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/product_book";
