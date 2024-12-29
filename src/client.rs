use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::Rng;
use serde::Serialize;

pub struct Client<'a> {
    pub name: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'_> {
    pub fn new(name: &'a str) -> Client<'a> {
        Client {
            name,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let result = self.client.get(url).send().await;
        if let Ok(response) = &result {
            if !response.status().is_success() {
                println!("Failed: {}", response.status());
            }
        } else if let Err(e) = &result {
            println!("Failed to get response: {}", e);
        }
        result
    }

    pub async fn get_auth(
        &self,
        url: &str,
        jwt: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let result = self.client.get(url).bearer_auth(jwt).send().await;
        if let Ok(response) = &result {
            if !response.status().is_success() {
                println!("Failed: {}", response.status());
            }
        } else if let Err(e) = &result {
            println!("Failed to get response: {}", e);
        }
        result
    }
}

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    iss: String,
    nbf: i64,
    exp: i64,
    uri: String,
    kid: String,
    nonce: String,
}

pub fn create_jwt(request_method: &str, request_path: &str) -> String {
    let key_name =
        std::env::var("CBAT_KEY_NAME").expect("CBAT_KEY_NAME environment variable not set");
    let key_secret =
        std::env::var("CBAT_KEY_SECRET").expect("CBAT_KEY_SECRET environment variable not set");
    let uri = format!("{} {}{}", request_method, BASE_URL, request_path);

    let mut rng = rand::thread_rng();
    let nonce: String = (0..16)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();

    let now = Utc::now();
    let claims = Claims {
        sub: key_name.to_owned(),
        iss: "cdp".to_owned(),
        nbf: now.timestamp(),
        exp: (now + Duration::seconds(60)).timestamp(),
        uri,
        kid: key_name.to_owned(),
        nonce,
    };
    let header = Header {
        alg: Algorithm::ES256,
        kid: Some(key_name.to_owned()),
        ..Default::default()
    };
    //let key_secret = key_secret.replace("\\n", "\n");
    let key = EncodingKey::from_ec_pem(key_secret.as_bytes()).expect("Invalid EC key");
    let jwt = encode(&header, &claims, &key).unwrap();
    jwt
}

pub const PROTOCOL: &str = "https://";
pub const BASE_URL: &str = "api.coinbase.com";
