use serde_derive::Deserialize;

use crate::{
    account::Account,
    client::{create_jwt, Client},
};

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Option<Vec<Account>>,
}

impl Accounts {
    pub async fn list_accounts(client: &Client<'_>) -> Result<Accounts, reqwest::Error> {
        let url = &format!("{}", PUBLIC_ACCOUNTS_URL);
        let response = client
            .get_auth(url, &create_jwt("GET", PUBLIC_ACCOUNTS_ENDPOINT))
            .await?;
        let products: Accounts = response.json().await?;
        Ok(products)
    }
}

const PUBLIC_ACCOUNTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/accounts";
pub const PUBLIC_ACCOUNTS_ENDPOINT: &str = "/api/v3/brokerage/accounts";
