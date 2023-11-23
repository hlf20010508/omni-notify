#[macro_export]
macro_rules! email {
    ( $data_type:ident ) => {
        use actix_web::{web, Responder};
        use lettre::message::header::ContentType;
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};
        use pico_args::Arguments;

        use crate::responses::{response_internal_server_error, response_text};
        use crate::structures::NotifyMailParams;

        use crate::get_value;

        pub async fn email(params: web::$data_type<NotifyMailParams>) -> impl Responder {
            let mut args = Arguments::from_env();
            let email_address: String = get_value!(args.value_from_str("--email"));
            let smtp_server_address: String = get_value!(args.value_from_str("--server"));
            let username: String = get_value!(args.value_from_str("--username"));
            let password: String = get_value!(args.value_from_str("--password"));
            
            let email = get_value!(Message::builder()
                .from(get_value!(email_address.parse()))
                .to(get_value!(email_address.parse()))
                .subject(&params.subject)
                .header(ContentType::TEXT_PLAIN)
                .body(params.body.clone()));

            let creds = Credentials::new(username, password);

            let mailer = get_value!(SmtpTransport::starttls_relay(&smtp_server_address))
                .credentials(creds)
                .build();

            let result = get_value!(mailer.send(&email));
            return response_text(result.message().collect());
        }
    };
}
