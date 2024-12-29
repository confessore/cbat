use serde_derive::Deserialize;

use crate::{client::Client, product::Product};

#[derive(Debug, Deserialize)]
pub struct Products {
    pub products: Option<Vec<Product>>,
}

impl Products {
    pub async fn list_public_products(
        client: &Client<'_>,
        limit: Option<u32>,
        offset: Option<u32>,
        product_type: Option<String>,
        products_ids: Option<Vec<String>>,
        contract_expiry_type: Option<String>,
        expiring_contract_status: Option<String>,
        get_all_products: Option<bool>,
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

        if let Some(products_ids) = products_ids {
            for product_id in products_ids {
                query_params.push(format!("product_ids={}", product_id));
            }
        }

        if let Some(contract_expiry_type) = contract_expiry_type {
            query_params.push(format!("contract_expiry_type={}", contract_expiry_type));
        }

        if let Some(expiring_contract_status) = expiring_contract_status {
            query_params.push(format!(
                "expiring_contract_status={}",
                expiring_contract_status
            ));
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
}

const PUBLIC_PRODUCTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/market/products";
