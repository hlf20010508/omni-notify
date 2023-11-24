#[macro_export]
macro_rules! get_value_or_response {
    ( $key:expr ) => {
        match $key {
            Ok(value) => value,
            Err(error_info) => return response_internal_server_error(error_info.to_string()),
        }
    };
}

#[macro_export]
macro_rules! get_value_or_error {
    ( $response_raw:expr, $err_kind:path ) => {
        match $response_raw {
            Ok(value) => value,
            Err(error_info) => return Err(Error::new($err_kind, error_info.to_string())),
        }
    };
}

#[macro_export]
macro_rules! func_crates {
    () => {
        use actix_web::{web, Responder};
        use lettre::message::header::ContentType;
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};
        use pico_args::Arguments;

        use crate::responses::{response_text, response_internal_server_error};
        use crate::requests;
        use crate::structures::{MailParams, PushParams, RequestPushParams};

        use crate::get_value_or_response;
    };
}

#[macro_export]
macro_rules! func_builder {
    ( $( $macro:ident ), * ) => {
        pub mod get {
            crate::func_crates!();
            $(
                use crate::$macro;
                $macro!(Query);
            )*

        }

        pub mod post {
            pub mod json {
                crate::func_crates!();
                $(
                    use crate::$macro;
                    $macro!(Json);
                )*
            }

            pub mod form {
                crate::func_crates!();
                $(
                    use crate::$macro;
                    $macro!(Form);
                )*
            }
        }
    };
}
