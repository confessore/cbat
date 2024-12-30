use serde_derive::Deserialize;

use crate::account::Account;

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub account: Option<Account>,
    pub accounts: Option<Vec<Account>>,
}
