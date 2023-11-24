#[macro_export]
macro_rules! push {
    ( $data_type:ident ) => {
        pub async fn push(params: web::$data_type<PushParams>) -> impl Responder {
            let mut args = Arguments::from_env();
            let pushkey = get_value_or_response!(args.value_from_str("--pushkey"));
            let url = String::from("https://api2.pushdeer.com/message/push");

            let params = RequestPushParams {
                pushkey,
                text: params.text.clone(),
            };

            let result = get_value_or_response!(requests::get(&url, params).await);
            return response_text(result);
        }
    };
}
