use crate::{
    cancel_orders::CancelOrders,
    client::{ create_jwt, Client },
    create_order::CreateOrder,
    create_order_request::CreateOrderRequest,
    edit_order::EditOrder,
    edit_order_request::EditOrderRequest,
    fills::Fills,
    http_method::HttpMethod,
    order_placement_source::OrderPlacementSource,
    order_side::OrderSide,
    orders::Orders,
    prelude::ContractExpiryType,
    preview_edit_order::PreviewEditOrder,
    preview_edit_order_request::PreviewEditOrderRequest,
    preview_order::PreviewOrder,
    preview_order_request::PreviewOrderRequest,
    product_type::ProductType,
    sort_by::SortBy,
};
use chrono::{ DateTime, Utc };
use serde_json::json;

pub struct ApiOrders;

impl ApiOrders {
    pub async fn cancel_orders(
        client: &Client<'_>,
        order_ids: Vec<&str>
    ) -> Result<CancelOrders, reqwest::Error> {
        let data = json!({
            "order_ids": order_ids,
        });
        let url = &format!("{}", CANCEL_ORDERS_URL);
        let response = client.post_auth(
            url,
            &create_jwt(HttpMethod::Post.as_str(), CANCEL_ORDERS_ENDPOINT),
            &data.to_string()
        ).await?;
        let cancel_orders: CancelOrders = response.json().await?;
        Ok(cancel_orders)
    }

    pub async fn create_order(
        client: &Client<'_>,
        request: CreateOrderRequest<'_>
    ) -> Result<CreateOrder, reqwest::Error> {
        let data = json!(request);
        let url = &format!("{}", CREATE_ORDER_URL);
        let response = client.post_auth(
            url,
            &create_jwt(HttpMethod::Post.as_str(), CREATE_ORDER_ENDPOINT),
            &data.to_string()
        ).await?;
        let create_order: CreateOrder = response.json().await?;
        Ok(create_order)
    }

    pub async fn preview_order(
        client: &Client<'_>,
        request: PreviewOrderRequest<'_>
    ) -> Result<PreviewOrder, reqwest::Error> {
        let data = json!(request);
        let url = &format!("{}", PREVIEW_ORDER_URL);
        let response = client.post_auth(
            url,
            &create_jwt(HttpMethod::Post.as_str(), PREVIEW_ORDER_ENDPOINT),
            &data.to_string()
        ).await?;
        let preview_order: PreviewOrder = response.json().await?;
        Ok(preview_order)
    }

    pub async fn preview_edit_order(
        client: &Client<'_>,
        request: PreviewEditOrderRequest<'_>
    ) -> Result<PreviewEditOrder, reqwest::Error> {
        let data = json!(request);
        let url = &format!("{}", PREVIEW_EDIT_ORDER_URL);
        let response = client.post_auth(
            url,
            &create_jwt(HttpMethod::Post.as_str(), PREVIEW_EDIT_ORDER_ENDPOINT),
            &data.to_string()
        ).await?;
        let preview_edit_order: PreviewEditOrder = response.json().await?;
        Ok(preview_edit_order)
    }

    pub async fn edit_order(
        client: &Client<'_>,
        request: EditOrderRequest<'_>
    ) -> Result<EditOrder, reqwest::Error> {
        let data = json!(request);
        let url = &format!("{}", EDIT_ORDER_URL);
        let response = client.post_auth(
            url,
            &create_jwt(HttpMethod::Post.as_str(), EDIT_ORDER_ENDPOINT),
            &data.to_string()
        ).await?;
        let edit_order: EditOrder = response.json().await?;
        Ok(edit_order)
    }

    pub async fn list_orders(
        client: &Client<'_>,
        order_ids: Option<Vec<&str>>,
        product_ids: Option<Vec<&str>>,
        product_type: Option<ProductType>,
        order_status: Option<Vec<&str>>,
        time_in_forces: Option<Vec<&str>>,
        order_types: Option<Vec<&str>>,
        order_side: Option<OrderSide>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        order_placement_source: Option<OrderPlacementSource>,
        contract_expiry_type: Option<ContractExpiryType>,
        asset_filters: Option<Vec<&str>>,
        retail_portfolio_id: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
        sort_by: Option<SortBy>,
        user_native_currency: Option<&str>
    ) -> Result<Orders, reqwest::Error> {
        let mut query_params = Vec::new();

        if let Some(order_ids) = order_ids {
            for order_id in order_ids {
                query_params.push(format!("order_ids={}", order_id));
            }
        }

        if let Some(product_ids) = product_ids {
            for product_id in product_ids {
                query_params.push(format!("product_ids={}", product_id));
            }
        }

        if let Some(product_type) = product_type {
            query_params.push(format!("product_type={}", product_type));
        }

        if let Some(order_status) = order_status {
            for status in order_status {
                query_params.push(format!("order_status={}", status));
            }
        }

        if let Some(time_in_forces) = time_in_forces {
            for time_in_force in time_in_forces {
                query_params.push(format!("time_in_forces={}", time_in_force));
            }
        }

        if let Some(order_types) = order_types {
            for order_type in order_types {
                query_params.push(format!("order_types={}", order_type));
            }
        }

        if let Some(order_side) = order_side {
            query_params.push(format!("order_side={}", order_side));
        }

        if let Some(start_date) = start_date {
            query_params.push(format!("start_date={}", start_date));
        }

        if let Some(end_date) = end_date {
            query_params.push(format!("end_date={}", end_date));
        }

        if let Some(order_placement_source) = order_placement_source {
            query_params.push(format!("order_placement_source={}", order_placement_source));
        }

        if let Some(contract_expiry_type) = contract_expiry_type {
            query_params.push(format!("contract_expiry_type={}", contract_expiry_type));
        }

        if let Some(asset_filters) = asset_filters {
            for asset_filter in asset_filters {
                query_params.push(format!("asset_filters={}", asset_filter));
            }
        }

        if let Some(retail_portfolio_id) = retail_portfolio_id {
            query_params.push(format!("retail_portfolio_id={}", retail_portfolio_id));
        }

        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if let Some(sort_by) = sort_by {
            query_params.push(format!("sort_by={}", sort_by));
        }

        if let Some(user_native_currency) = user_native_currency {
            query_params.push(format!("user_native_currency={}", user_native_currency));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}{}", LIST_ORDERS_URL, query_string);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), LIST_ORDERS_ENDPOINT)
        ).await?;
        let orders: Orders = response.json().await?;
        Ok(orders)
    }

    pub async fn get_order(
        client: &Client<'_>,
        order_id: &str,
        client_order_id: Option<&str>,
        user_native_currency: Option<&str>
    ) -> Result<Orders, reqwest::Error> {
        let mut query_params = Vec::new();
        if let Some(client_order_id) = client_order_id {
            query_params.push(format!("client_order_id={}", client_order_id));
        }
        if let Some(user_native_currency) = user_native_currency {
            query_params.push(format!("user_native_currency={}", user_native_currency));
        }
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}/{}{}", GET_ORDER_URL, order_id, query_string);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), &format!("{}/{}", GET_ORDER_ENDPOINT, order_id))
        ).await?;
        let orders: Orders = response.json().await?;
        Ok(orders)
    }

    pub async fn list_fills(
        client: &Client<'_>,
        order_ids: Option<Vec<&str>>,
        trade_ids: Option<Vec<&str>>,
        product_ids: Option<Vec<&str>>,
        start_sequence_timestamp: Option<DateTime<Utc>>,
        end_sequence_timestamp: Option<DateTime<Utc>>,
        retail_portfolio_id: Option<&str>,
        limit: Option<u64>,
        cursor: Option<&str>,
        sort_by: Option<SortBy>
    ) -> Result<Fills, reqwest::Error> {
        let mut query_params = Vec::new();
        if let Some(order_ids) = order_ids {
            for order_id in order_ids {
                query_params.push(format!("order_ids={}", order_id));
            }
        }
        if let Some(trade_ids) = trade_ids {
            for trade_id in trade_ids {
                query_params.push(format!("trade_ids={}", trade_id));
            }
        }
        if let Some(product_ids) = product_ids {
            for product_id in product_ids {
                query_params.push(format!("product_ids={}", product_id));
            }
        }
        if let Some(start_sequence_timestamp) = start_sequence_timestamp {
            query_params.push(format!("start_sequence_timestamp={}", start_sequence_timestamp));
        }
        if let Some(end_sequence_timestamp) = end_sequence_timestamp {
            query_params.push(format!("end_sequence_timestamp={}", end_sequence_timestamp));
        }
        if let Some(retail_portfolio_id) = retail_portfolio_id {
            query_params.push(format!("retail_portfolio_id={}", retail_portfolio_id));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(cursor) = cursor {
            query_params.push(format!("cursor={}", cursor));
        }
        if let Some(sort_by) = sort_by {
            query_params.push(format!("sort_by={}", sort_by));
        }
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        let url = &format!("{}{}", LIST_FILLS_URL, query_string);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), LIST_FILLS_ENDPOINT)
        ).await?;
        let orders: Fills = response.json().await?;
        Ok(orders)
    }
}

const CANCEL_ORDERS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel";
const CANCEL_ORDERS_ENDPOINT: &str = "/api/v3/brokerage/orders/batch_cancel";
const CREATE_ORDER_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders";
const CREATE_ORDER_ENDPOINT: &str = "/api/v3/brokerage/orders";
const PREVIEW_ORDER_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/preview";
const PREVIEW_ORDER_ENDPOINT: &str = "/api/v3/brokerage/orders/preview";
const PREVIEW_EDIT_ORDER_URL: &str =
    "https://api.coinbase.com/api/v3/brokerage/orders/edit_preview";
const PREVIEW_EDIT_ORDER_ENDPOINT: &str = "/api/v3/brokerage/orders/edit_preview";
const EDIT_ORDER_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/edit";
const EDIT_ORDER_ENDPOINT: &str = "/api/v3/brokerage/orders/edit";
const LIST_ORDERS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/historical/batch";
const LIST_ORDERS_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";
const GET_ORDER_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/historical";
const GET_ORDER_ENDPOINT: &str = "/api/v3/brokerage/orders/historical";
const LIST_FILLS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/orders/historical/fills";
const LIST_FILLS_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/fills";
