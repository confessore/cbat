use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Portfolio {
    pub name: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "type")]
    pub o_type: Option<String>,
    pub deleted: Option<bool>,
}
