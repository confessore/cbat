use serde_derive::Deserialize;

use crate::client::Client;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ServerTime {
    pub iso: Option<String>,
    pub epochSeconds: Option<String>,
    pub epochMillis: Option<String>,
}

impl ServerTime {
    pub async fn get_public_server_time(client: &Client<'_>) -> Result<ServerTime, reqwest::Error> {
        let response = client
            .get("https://api.coinbase.com/api/v3/brokerage/time")
            .await?;
        let server_time: ServerTime = response.json().await?;
        Ok(server_time)
    }
}
