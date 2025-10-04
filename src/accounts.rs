use serde::{Deserialize, Serialize};

use crate::account::Account;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accounts {
    pub account: Option<Account>,
    pub accounts: Option<Vec<Account>>,
}
