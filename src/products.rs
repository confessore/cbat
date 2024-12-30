use serde_derive::Deserialize;

use crate::{
    client::Client,
    contract_expiry_type::ContractExpiryType,
    expiring_contract_status::ExpiringContractStatus,
    product::Product,
    product_type::ProductType,
};

#[derive(Debug, Deserialize)]
pub struct Products {
    pub products: Option<Vec<Product>>,
}
