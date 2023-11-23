#[macro_export]
macro_rules! get_value {
    ( $key:expr ) => {
        match $key {
            Ok(value) => value,
            Err(error_info) => return response_internal_server_error(error_info.to_string()),
        }
    };
}

#[macro_export]
macro_rules! func_builder {
    ( $macro:ident ) => {
        pub mod get {
            pub mod notify {
                use crate::$macro;
                $macro!(Query);
            }
        }

        pub mod post {
            pub mod json {
                pub mod notify {
                    use crate::$macro;
                    $macro!(Json);
                }
            }

            pub mod form {
                pub mod notify {
                    use crate::$macro;
                    $macro!(Form);
                }
            }
        }
    };
}
