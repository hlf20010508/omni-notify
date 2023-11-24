use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct MailParams {
    pub subject: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct PushParams {
    pub text: String,
}

#[derive(Serialize)]
pub struct RequestPushParams {
    pub pushkey: String,
    pub text: String,
}