use crate::{ accounts::Accounts, client::{ create_jwt, Client }, http_method::HttpMethod };

pub struct ApiAccounts;

impl ApiAccounts {
    pub async fn list_accounts(client: &Client<'_>) -> Result<Accounts, reqwest::Error> {
        let url = &format!("{}", ACCOUNTS_URL);
        let response = client.get_auth(
            url,
            &create_jwt(HttpMethod::Get.as_str(), ACCOUNTS_ENDPOINT)
        ).await?;
        let accounts: Accounts = response.json().await?;
        Ok(accounts)
    }

    pub async fn get_account(
        client: &Client<'_>,
        account_uuid: &str
    ) -> Result<Accounts, reqwest::Error> {
        let url = &format!("{}/{}", ACCOUNTS_URL, account_uuid);
        let response = client.get_auth(
            url,
            &create_jwt(
                HttpMethod::Get.as_str(),
                &format!("{}/{}", ACCOUNTS_ENDPOINT, account_uuid)
            )
        ).await?;
        let accounts: Accounts = response.json().await?;
        Ok(accounts)
    }
}

const ACCOUNTS_URL: &str = "https://api.coinbase.com/api/v3/brokerage/accounts";
const ACCOUNTS_ENDPOINT: &str = "/api/v3/brokerage/accounts";
