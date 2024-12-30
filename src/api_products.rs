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
}

const BEST_BID_ASK_URL: &str = "https://api.coinbase.com/api/v3/brokerage/best_bid_ask";
const BEST_BID_ASK_ENDPOINT: &str = "/api/v3/brokerage/best_bid_ask";
