use crate::product_type::ProductType;

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
    use crate::{client::Client, products::Products};
    let client = Client::new(EXAMPLE);
    let products =
        Products::list_public_products(&client, Some(1), None, Some(ProductType::Spot), None, None, None, None).await;
    assert_eq!(products.is_ok(), true);
}
