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
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let market_trades = client.get_public_market_trades("BTC-USD", 1, None, None).await;
    assert_eq!(market_trades.is_ok(), true);
}

#[tokio::test]
pub async fn product_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let product = client.get_public_product("BTC-USD").await;
    assert_eq!(product.is_ok(), true);
}

#[tokio::test]
pub async fn product_book_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let product_book = client.get_public_product_book("BTC-USD", Some(1), None).await;
    assert_eq!(product_book.is_ok(), true);
}

#[tokio::test]
pub async fn server_time_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let server_time = client.get_public_server_time().await;
    assert_eq!(server_time.is_ok(), true);
}

#[tokio::test]
pub async fn products_test() {
    use crate::{
        client::Client,
        contract_expiry_type::ContractExpiryType,
        expiring_contract_status::ExpiringContractStatus,
        product_type::ProductType,
    };
    let client = Client::new(EXAMPLE);
    let products = client.list_public_products(
        Some(1),
        None,
        Some(ProductType::Future),
        None,
        Some(ContractExpiryType::Expiring),
        Some(ExpiringContractStatus::Unexpired),
        None
    ).await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn product_candles_test() {
    use crate::{ client::Client, granularity::Granularity };
    use chrono::{ DateTime, Utc };
    let client = Client::new(EXAMPLE);
    let current_time: DateTime<Utc> = Utc::now();
    let epoch_time = current_time.timestamp();
    let start = epoch_time - 10_000;
    let end = epoch_time + 10_000;
    let products = client.get_public_product_candles(
        "BTC-USD",
        &start.to_string(),
        &end.to_string(),
        Granularity::OneMinute,
        Some(1)
    ).await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn accounts_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let account = client.list_accounts().await;
    assert_eq!(account.is_ok(), true);
}

#[tokio::test]
pub async fn account_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let accounts = client.list_accounts().await.unwrap();
    let account_uuid = &accounts.accounts.unwrap()[0].uuid;
    let accounts = client.get_account(&account_uuid).await;
    assert_eq!(accounts.is_ok(), true);
}

#[tokio::test]
pub async fn best_bid_ask_test() {
    use crate::client::Client;
    let client = Client::new(EXAMPLE);
    let price_books = client.get_best_bid_ask(Some(vec!["BTC-USD"])).await;
    assert_eq!(price_books.is_ok(), true);
}
