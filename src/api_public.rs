use crate::{
    client::Client,
    contract_expiry_type::ContractExpiryType,
    expiring_contract_status::ExpiringContractStatus,
    granularity::Granularity,
    market_trades::MarketTrades,
    product::Product,
    product_book::ProductBook,
    product_candles::ProductCandles,
    product_type::ProductType,
    products::Products,
    server_time::ServerTime,
};

pub struct ApiPublic;

impl ApiPublic {
    pub async fn get_public_market_trades(
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
            "{}{}/ticker?limit={}{}{}",
            PUBLIC_MARKET_TRADES_URL,
            product_id,
            limit,
            start,
            end
        );
        let response = client.get(url).await?;
        let market_trades: MarketTrades = response.json().await?;
        Ok(market_trades)
    }

    pub async fn get_public_product_book(
        client: &Client<'_>,
        product_id: &str,
        limit: Option<u32>,
        aggregation_price_increment: Option<&str>
    ) -> Result<ProductBook, reqwest::Error> {
        let limit = match limit {
            Some(limit) => &format!("&limit={}", limit),
            None => "",
        };
        let aggregation_price_increment = match aggregation_price_increment {
            Some(aggregation_price_increment) =>
                &format!("&aggregation_price_increment={}", aggregation_price_increment),
            None => "",
        };
        let url = &format!(
            "{}?product_id={}{}{}",
            PUBLIC_PRODUCT_BOOK_URL,
            product_id,
            limit,
            aggregation_price_increment
        );
        let response = client.get(url).await?;
        let product: ProductBook = response.json().await?;
        Ok(product)
    }

    pub async fn get_public_product_candles(
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
            PUBLIC_PRODUCT_URL,
            product_id,
            start,
            end,
            granularity,
            limit
        );
        let response = client.get(url).await?;
        let product_candles: ProductCandles = response.json().await?;
        Ok(product_candles)
    }

    pub async fn get_public_product(
        client: &Client<'_>,
        product_id: &str
    ) -> Result<Product, reqwest::Error> {
        let url = &format!("{}/{}", PUBLIC_PRODUCT_URL, product_id);
        let response = client.get(url).await?;
        let product: Product = response.json().await?;
        Ok(product)
    }
    pub async fn list_public_products(
        client: &Client<'_>,
        limit: Option<u32>,
        offset: Option<u32>,
        product_type: Option<ProductType>,
        product_ids: Option<Vec<&str>>,
        contract_expiry_type: Option<ContractExpiryType>,
        expiring_contract_status: Option<ExpiringContractStatus>,
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

        if let Some(get_all_products) = get_all_products {
            query_params.push(format!("get_all_products={}", get_all_products));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let url = &format!("{}{}", PUBLIC_PRODUCTS_URL, query_string);
        let response = client.get(url).await?;
        let products: Products = response.json().await?;
        Ok(products)
    }

    pub async fn get_public_server_time(client: &Client<'_>) -> Result<ServerTime, reqwest::Error> {
        let response = client.get(PUBLIC_SERVER_TIME).await?;
        let server_time: ServerTime = response.json().await?;
        Ok(server_time)
    }
}

const PUBLIC_MARKET_TRADES_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products/";
const PUBLIC_MARKET_TRADES_ENDPOINT: &str = "/api/v3/brokerage/market/products/ticker";
const PUBLIC_PRODUCT_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
const PUBLIC_PRODUCT_ENDPOINT: &str = "/api/v3/brokerage/market/products";
const PUBLIC_PRODUCTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
const PUBLIC_PRODUCTS_ENDPOINT: &str = "/api/v3/brokerage/market/products";
const PUBLIC_PRODUCT_BOOK_URL: &str =
    "https://api.coinbase.com/api/v3/brokerage/market/product_book";
const PUBLIC_SERVER_TIME: &str = "https://api.coinbase.com/api/v3/brokerage/time";
