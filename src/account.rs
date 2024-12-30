use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub uuid: String,
}
