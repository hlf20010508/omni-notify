use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct MailParams {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PushParams {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct RequestPushParams {
    pub pushkey: String,
    pub text: String,
    pub desp: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}
