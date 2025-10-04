#[cfg(test)]
const EXAMPLE: &str = "example";

#[test]
pub fn client_test() {
    use crate::client::Client;
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    assert_eq!(client.name, EXAMPLE);
}

#[tokio::test]
pub async fn public_market_trades_test() {
    use crate::{api_public::ApiPublic, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let market_trades =
        ApiPublic::get_public_market_trades(&client, "BTC-USD", 1, None, None).await;
    assert_eq!(market_trades.is_ok(), true);
}

#[tokio::test]
pub async fn public_product_test() {
    use crate::{api_public::ApiPublic, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let product = ApiPublic::get_public_product(&client, "BTC-USD").await;
    assert_eq!(product.is_ok(), true);
}

#[tokio::test]
pub async fn public_product_book_test() {
    use crate::{api_public::ApiPublic, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let product_book = ApiPublic::get_public_product_book(&client, "BTC-USD", Some(1), None).await;
    assert_eq!(product_book.is_ok(), true);
}

#[tokio::test]
pub async fn public_server_time_test() {
    use crate::{api_public::ApiPublic, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let server_time = ApiPublic::get_public_server_time(&client).await;
    assert_eq!(server_time.is_ok(), true);
}

#[tokio::test]
pub async fn public_products_test() {
    use crate::{
        api_public::ApiPublic, client::Client, contract_expiry_type::ContractExpiryType,
        expiring_contract_status::ExpiringContractStatus, product_type::ProductType,
    };
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let products = ApiPublic::list_public_products(
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
pub async fn public_product_candles_test() {
    use crate::{api_public::ApiPublic, client::Client, granularity::Granularity};
    use chrono::{DateTime, Utc};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let current_time: DateTime<Utc> = Utc::now();
    let epoch_time = current_time.timestamp();
    let start = epoch_time - 10_000;
    let end = epoch_time + 10_000;
    let products = ApiPublic::get_public_product_candles(
        &client,
        "BTC-USD",
        &start.to_string(),
        &end.to_string(),
        Granularity::OneMinute,
        Some(1),
    )
    .await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn accounts_test() {
    use crate::{api_accounts::ApiAccounts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let account = ApiAccounts::list_accounts(&client).await;
    assert_eq!(account.is_ok(), true);
}

#[tokio::test]
pub async fn account_test() {
    use crate::{api_accounts::ApiAccounts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let accounts = ApiAccounts::list_accounts(&client).await.unwrap();
    let account_uuid = &accounts.accounts.unwrap()[0].uuid;
    let accounts = ApiAccounts::get_account(&client, &account_uuid).await;
    assert_eq!(accounts.is_ok(), true);
}

#[tokio::test]
pub async fn best_bid_ask_test() {
    use crate::{api_products::ApiProducts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let price_books = ApiProducts::get_best_bid_ask(&client, Some(vec!["BTC-USD"])).await;
    assert_eq!(price_books.is_ok(), true);
}

#[tokio::test]
pub async fn product_book_test() {
    use crate::{api_products::ApiProducts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let price_books = ApiProducts::get_product_book(&client, "BTC-USD", Some(1), None).await;
    assert_eq!(price_books.is_ok(), true);
}

#[tokio::test]
pub async fn products_test() {
    use crate::{api_products::ApiProducts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let products =
        ApiProducts::list_products(&client, Some(1), None, None, None, None, None, None, None)
            .await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn product_test() {
    use crate::{api_products::ApiProducts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let products = ApiProducts::get_product(&client, "BTC-USD", None).await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn product_candles_test() {
    use crate::{api_products::ApiProducts, client::Client, granularity::Granularity};
    use chrono::{DateTime, Utc};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let current_time: DateTime<Utc> = Utc::now();
    let epoch_time = current_time.timestamp();
    let start = epoch_time - 10_000;
    let end = epoch_time + 10_000;
    let products = ApiProducts::get_product_candles(
        &client,
        "BTC-USD",
        &start.to_string(),
        &end.to_string(),
        Granularity::OneMinute,
        Some(1),
    )
    .await;
    assert_eq!(products.is_ok(), true);
}

#[tokio::test]
pub async fn market_trades_test() {
    use crate::{api_products::ApiProducts, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let market_trades = ApiProducts::get_market_trades(&client, "BTC-USD", 1, None, None).await;
    assert_eq!(market_trades.is_ok(), true);
}

#[tokio::test]
pub async fn portfolios_test() {
    use crate::{api_portfolios::ApiPortfolios, client::Client, portfolio_type::PortfolioType};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let portfolios = ApiPortfolios::list_portfolios(&client, Some(PortfolioType::Undefined)).await;
    assert_eq!(portfolios.is_ok(), true);
}

#[tokio::test]
pub async fn preview_order_test() {
    use crate::{
        api_orders::ApiOrders, client::Client, limit_limit_gtc::LimitLimitGtc,
        order_configuration::OrderConfiguration, preview_order_request::PreviewOrderRequest,
    };
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let request = PreviewOrderRequest {
        product_id: "BTC-USD",
        side: "SELL",
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_fok: None,
            limit_limit_gtd: None,
            limit_limit_gtc: Some(LimitLimitGtc {
                quote_size: None,
                base_size: Some((0.001).to_string()),
                limit_price: Some((999_999.0).to_string()),
                post_only: None,
            }),
            sor_limit_ioc: None,
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: None,
            trigger_bracket_gtc: None,
            trigger_bracket_gtd: None,
        },
        leverage: None,
        margin_type: None,
        retail_portfolio_id: None,
    };
    let preview_order = ApiOrders::preview_order(&client, request).await;
    assert_eq!(preview_order.is_ok(), true);
}

#[tokio::test]
pub async fn create_edit_cancel_order_test() {
    use crate::{
        api_orders::ApiOrders, client::Client, create_order_request::CreateOrderRequest,
        edit_order_request::EditOrderRequest, limit_limit_gtc::LimitLimitGtc,
        order_configuration::OrderConfiguration,
        preview_edit_order_request::PreviewEditOrderRequest,
    };
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let request = CreateOrderRequest {
        client_order_id: &uuid::Uuid::new_v4().to_string(),
        product_id: "BTC-USD",
        side: "SELL",
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_fok: None,
            limit_limit_gtd: None,
            limit_limit_gtc: Some(LimitLimitGtc {
                quote_size: None,
                base_size: Some((0.001).to_string()),
                limit_price: Some((999_999.0).to_string()),
                post_only: None,
            }),
            sor_limit_ioc: None,
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: None,
            trigger_bracket_gtc: None,
            trigger_bracket_gtd: None,
        },
        leverage: None,
        margin_type: None,
        retail_portfolio_id: None,
        preview_id: None,
    };
    let create_order = ApiOrders::create_order(&client, request).await;
    assert_eq!(create_order.is_ok(), true);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let order_id = create_order.unwrap().success_response.order_id;
    let request = PreviewEditOrderRequest {
        order_id: &order_id,
        price: (999_998.0).to_string(),
        size: (0.001).to_string(),
    };
    let preview_edit_order = ApiOrders::preview_edit_order(&client, request).await;
    assert_eq!(preview_edit_order.is_ok(), true);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let request = EditOrderRequest {
        order_id: &order_id,
        price: (999_998.0).to_string(),
        size: (0.001).to_string(),
    };
    let edit_order = ApiOrders::edit_order(&client, request).await;
    assert_eq!(edit_order.is_ok(), true);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let order = ApiOrders::get_order(&client, &order_id, None, None).await;
    assert_eq!(order.is_ok(), true);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let cancel_orders = ApiOrders::cancel_orders(&client, vec![&order_id]).await;
    assert_eq!(cancel_orders.is_ok(), true);
}

#[tokio::test]
pub async fn list_orders_test() {
    use crate::{api_orders::ApiOrders, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let orders = ApiOrders::list_orders(
        &client,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&(1).to_string()),
        None,
        None,
        None,
    )
    .await;
    assert_eq!(orders.is_ok(), true);
}

#[tokio::test]
pub async fn list_fills_test() {
    use crate::{api_orders::ApiOrders, client::Client};
    let client = Client::new(
        EXAMPLE,
        std::env::var("CBAT_KEY_NAME").unwrap_or_default(),
        std::env::var("CBAT_KEY_SECRET").unwrap_or_default(),
    );
    let fills = ApiOrders::list_fills(
        &client,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(1),
        None,
        None,
    )
    .await;
    assert_eq!(fills.is_ok(), true);
}
