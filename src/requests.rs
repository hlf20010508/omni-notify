use async_std::io::{Error, ErrorKind, Result};
use awc::{Client, Connector};

use crate::structures::RequestPushParams;

use crate::get_value_or_error;

pub async fn get(url: &str, params: RequestPushParams) -> Result<String> {
    let client = Client::builder().connector(Connector::new()).finish();

    let request = get_value_or_error!(
        client
            .get(url)
            .insert_header(("User-Agent", "Actix-web"))
            .query(&params),
        ErrorKind::InvalidInput
    );

    let mut response = get_value_or_error!(request.send().await, ErrorKind::ConnectionRefused);

    let response_body = get_value_or_error!(response.body().await, ErrorKind::NotFound);

    let body_content = get_value_or_error!(
        String::from_utf8(response_body.to_vec()),
        ErrorKind::InvalidData
    );

    return Ok(body_content);
}
