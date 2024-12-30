use serde_derive::Deserialize;

use crate::client::{create_jwt, Client};

#[derive(Debug, Deserialize)]
pub struct Account {
    pub uuid: String,
}