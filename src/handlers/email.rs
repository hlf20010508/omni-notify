#[macro_export]
macro_rules! email {
    ( $data_type:ident ) => {
        pub async fn email(params: web::$data_type<MailParams>) -> impl Responder {
            let mut args = Arguments::from_env();
            let email_address: String = get_value_or_response!(args.value_from_str("--email"));
            let smtp_server_address: String = get_value_or_response!(args.value_from_str("--server"));
            let username: String = get_value_or_response!(args.value_from_str("--username"));
            let password: String = get_value_or_response!(args.value_from_str("--password"));
            
            let email = get_value_or_response!(Message::builder()
                .from(get_value_or_response!(email_address.parse()))
                .to(get_value_or_response!(email_address.parse()))
                .subject(&params.title)
                .header(ContentType::TEXT_PLAIN)
                .body(params.body.clone()));

            let creds = Credentials::new(username, password);

            let mailer = get_value_or_response!(SmtpTransport::starttls_relay(&smtp_server_address))
                .credentials(creds)
                .build();

            let result = get_value_or_response!(mailer.send(&email));
            return response_text(result.message().collect());
        }
    };
}
