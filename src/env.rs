use lazy_static::lazy_static;
use pico_args::{Arguments, Error};

fn get_arg_value(arg_name: &'static str) -> Result<String, Error> {
    let mut args = Arguments::from_env();
    let value: String = args.value_from_str(arg_name)?;
    return Ok(value);
}

lazy_static! {
    pub static ref EMAIL: Result<String, Error> = get_arg_value("--email");
    pub static ref SMTP_SERVER: Result<String, Error> = get_arg_value("--smtp-server");
    pub static ref SMTP_USERNAME: Result<String, Error> = get_arg_value("--smtp-username");
    pub static ref SMTP_PASSWORD: Result<String, Error> = get_arg_value("--smtp-password");
    pub static ref PUSHDEER_KEY: Result<String, Error> = get_arg_value("--pushdeer-key");
    pub static ref TG_BOT_TOKEN: Result<String, Error> = get_arg_value("--tg-bot-token");
    pub static ref TG_CHAT_ID: Result<String, Error> = get_arg_value("--tg-chat-id");
}
