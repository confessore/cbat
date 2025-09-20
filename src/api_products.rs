use crate::{
    client::Client,
    contract_expiry_type::ContractExpiryType,
    expiring_contract_status::ExpiringContractStatus,
    granularity::Granularity,
    http_method::HttpMethod,
    market_trades::MarketTrades,
    price_books::PriceBooks,
    product::Product,
    product_candles::ProductCandles,
    products::Products,
};

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
            &client.create_jwt(HttpMethod::Get.as_str(), BEST_BID_ASK_ENDPOINT)
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
            &client.create_jwt(HttpMethod::Get.as_str(), PRODUCT_BOOK_ENDPOINT)
        ).await?;
        let price_books: PriceBooks = response.json().await?;
        Ok(price_books)
    }

    pub async fn list_products(
        client: &Client<'_>,
        limit: Option<u32>,
        offset: Option<u32>,
        product_type: Option<&str>,
        product_ids: Option<Vec<&str>>,
        contract_expiry_type: Option<ContractExpiryType>,
        expiring_contract_status: Option<ExpiringContractStatus>,
        get_tradability_status: Option<bool>,
        get_all_products: Option<bool>
    ) -> Result<Products, reqwest::Error> {
        let mut query_params = Vec::new();
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(offset) = offset {
            query_params.push(format!("offset={}", offset));
        }
        if let Some(product_type) = product_type {
            query_params.push(format!("product_type={}", product_type));
        }
        if let Some(product_ids) = product_ids {
            for product_id in product_ids {
                query_params.push(format!("product_ids={}", product_id));
            }
        }
        if let Some(contract_expiry_type) = contract_expiry_type {
            query_params.push(format!("contract_expiry_type={}", contract_expiry_type));
        }
        if let Some(expiring_contract_status) = expiring_contract_status {
            query_params.push(format!("expiring_contract_status={}", expiring_contract_status));
        }
        if let Some(get_tradability_status) = get_tradability_status {
            query_params.push(format!("get_tradability_status={}", get_tradability_status));
        }
        if let Some(get_all_products) = get_all_products {
            query_params.push(format!("get_all_products={}", get_all_products));
        }
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}{}", PRODUCTS_URL, query_string);
        let response = client.get_auth(
            url,
            &client.create_jwt(HttpMethod::Get.as_str(), PRODUCTS_ENDPOINT)
        ).await?;
        let products: Products = response.json().await?;
        Ok(products)
    }

    pub async fn get_product(
        client: &Client<'_>,
        product_id: &str,
        get_tradability_status: Option<bool>
    ) -> Result<Product, reqwest::Error> {
        let get_tradability_status = match get_tradability_status {
            Some(get_tradability_status) =>
                &format!("?get_tradability_status={}", get_tradability_status),
            None => "",
        };
        let url = &format!("{}/{}{}", PRODUCTS_URL, product_id, get_tradability_status);
        let response = client.get_auth(
            url,
            &client.create_jwt(
                HttpMethod::Get.as_str(),
                &format!("{}/{}", PRODUCTS_ENDPOINT, product_id)
            )
        ).await?;
        let products: Product = response.json().await?;
        Ok(products)
    }

    pub async fn get_product_candles(
        client: &Client<'_>,
        product_id: &str,
        start: &str,
        end: &str,
        granularity: Granularity,
        limit: Option<u32>
    ) -> Result<ProductCandles, reqwest::Error> {
        let limit = match limit {
            Some(limit) => &format!("&limit={}", limit),
            None => "",
        };

        let url = &format!(
            "{}/{}/candles?start={}&end={}&granularity={}{}",
            PRODUCTS_URL,
            product_id,
            start,
            end,
            granularity,
            limit
        );
        let response = client.get_auth(
            url,
            &client.create_jwt(
                HttpMethod::Get.as_str(),
                &format!("{}/{}/candles", PRODUCTS_ENDPOINT, product_id)
            )
        ).await?;
        let product_candles: ProductCandles = response.json().await?;
        Ok(product_candles)
    }

    pub async fn get_market_trades(
        client: &Client<'_>,
        product_id: &str,
        limit: u32,
        start: Option<String>,
        end: Option<String>
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
            "{}/{}/ticker?limit={}{}{}",
            MARKET_TRADES_URL,
            product_id,
            limit,
            start,
            end
        );
        let response = client.get_auth(
            url,
            &client.create_jwt(
                HttpMethod::Get.as_str(),
                &format!("{}/{}/ticker", MARKET_TRADES_ENDPOINT, product_id)
            )
        ).await?;
        let market_trades: MarketTrades = response.json().await?;
        Ok(market_trades)
    }
}

const BEST_BID_ASK_URL: &str = "https://api.coinbase.com/api/v3/brokerage/best_bid_ask";
const BEST_BID_ASK_ENDPOINT: &str = "/api/v3/brokerage/best_bid_ask";
const PRODUCT_BOOK_URL: &str = "https://api.coinbase.com/api/v3/brokerage/product_book";
const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/product_book";
const PRODUCTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/products";
const PRODUCTS_ENDPOINT: &str = "/api/v3/brokerage/products";
const MARKET_TRADES_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
const MARKET_TRADES_ENDPOINT: &str = "/api/v3/brokerage/market/products";
