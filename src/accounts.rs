use serde_derive::Deserialize;

use crate::{
    account::Account,
    client::{create_jwt, Client},
};

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub account: Option<Account>,
    pub accounts: Option<Vec<Account>>,
}

impl Accounts {
    pub async fn list_accounts(client: &Client<'_>) -> Result<Accounts, reqwest::Error> {
        let url = &format!("{}", PUBLIC_ACCOUNTS_URL);
        let response = client
            .get_auth(url, &create_jwt("GET", PUBLIC_ACCOUNTS_ENDPOINT))
            .await?;
        let accounts: Accounts = response.json().await?;
        Ok(accounts)
    }

    pub async fn get_account(client: &Client<'_>, account_uuid: &str) -> Result<Accounts, reqwest::Error> {
        let url = &format!("{}/{}", PUBLIC_ACCOUNTS_URL, account_uuid);
        let response = client
            .get_auth(url, &create_jwt("GET", &format!("{}/{}", PUBLIC_ACCOUNTS_ENDPOINT, account_uuid)))
            .await?;
        let accounts: Accounts = response.json().await?;
        Ok(accounts)
    }
}

const PUBLIC_ACCOUNTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/accounts";
pub const PUBLIC_ACCOUNTS_ENDPOINT: &str = "/api/v3/brokerage/accounts";
