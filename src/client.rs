pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await
    }
}
