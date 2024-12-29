#[cfg(test)]

const EXAMPLE: &str = "example";

#[test]
pub fn client_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    assert_eq!(client.name, EXAMPLE);
}

#[tokio::test]
pub async fn market_trades_test() {
    use crate::{client::Client, market_trades::MarketTrades};
    let client = Client::new(EXAMPLE);
    let market_trades =
        MarketTrades::get_public_market_trades(&client, "BTC-USD", 1, None, None).await;
    assert_eq!(market_trades.is_ok(), true);
}

#[tokio::test]
pub async fn product_test() {
    use crate::{client::Client, product::Product};
    let client = Client::new(EXAMPLE);
    let product = Product::get_public_product(&client, "BTC-USD").await;
    assert_eq!(product.is_ok(), true);
}

#[tokio::test]
pub async fn product_book_test() {
    use crate::{client::Client, product_book::ProductBook};
    let client = Client::new(EXAMPLE);
    let product_book =
        ProductBook::get_public_product_book(&client, "BTC-USD", Some(1), None).await;
    assert_eq!(product_book.is_ok(), true);
}

#[tokio::test]
pub async fn server_time_test() {
    use crate::{client::Client, server_time::ServerTime};
    let client = Client::new(EXAMPLE);
    let server_time = ServerTime::get_public_server_time(&client).await;
    assert_eq!(server_time.is_ok(), true);
}

#[tokio::test]
pub async fn products_test() {
    use crate::{client::Client, products::Products, product_type::ProductType, contract_expiry_type::ContractExpiryType, expiring_contract_status::ExpiringContractStatus};
    let client = Client::new(EXAMPLE);
    let products = Products::list_public_products(
        &client,
        Some(1),
        None,
        Some(ProductType::Future),
        None,
        Some(ContractExpiryType::Expiring),
        Some(ExpiringContractStatus::Unexpired),
        None,
    )
    .await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]

pub async fn product_candles_test() {
    use crate::{client::Client, granularity::Granularity, product_candles::ProductCandles, server_time::ServerTime};
    let client = Client::new(EXAMPLE);
    let server_time_result = ServerTime::get_public_server_time(&client).await;
    let server_time = server_time_result.unwrap();
    let start_end = server_time.iso.unwrap();
    let products = ProductCandles::get_public_product_candles(
        &client,
        "BTC-USD",
        &start_end,
        &start_end,
        Granularity::OneMinute,
        Some(1),
    )
    .await;
    assert_eq!(products.is_ok(), true);
}
