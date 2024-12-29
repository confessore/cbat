pub struct Client<'a> {
    pub name: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(name: &'a str) -> Client<'a> {
        Client {
            name,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await
    }
}
