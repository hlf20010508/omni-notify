use serde::Deserialize;

#[derive(Deserialize)]
pub struct NotifyMailParams {
    pub subject: String,
    pub body: String,
}
