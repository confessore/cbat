use crate::{
    cancel_orders::CancelOrders,
    client::{ create_jwt, Client },
    create_order::CreateOrder,
    create_order_request::CreateOrderRequest,
    edit_order::EditOrder,
    http_method::HttpMethod,
    edit_order_request::EditOrderRequest,
    preview_edit_order_request::PreviewEditOrderRequest,
    preview_edit_order::PreviewEditOrder,
    preview_order::PreviewOrder,
    preview_order_request::PreviewOrderRequest,
};
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

    pub async fn list_orders(client: &Client<'_>) -> Result<reqwest::Response, reqwest::Error> {
        let url = &format!("{}", LIST_ORDERS_URL);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), LIST_ORDERS_ENDPOINT)
        ).await?;
        Ok(response)
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
