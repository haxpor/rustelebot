use isahc::prelude::*;
use url::Url;
use types::*;

#[cfg(test)]
pub mod tests;

pub mod types;
pub mod utils;
pub mod isahc_utils;

/// Create an instance to interact with APIs.
/// Return `BotInstance`.
///
/// # Arguments
/// * `bot_token` - a string of bot token
/// * `chat_id` - a chat id string to send message
pub fn create_instance(bot_token: &str, chat_id: &str) -> BotInstance {
    BotInstance { bot_token: bot_token.to_string(), chat_id: chat_id.to_string() }
}

/// Send message.
/// Return `Result<(), ErrorResult>`.
///
/// # Arguments
///
/// * `instance` - `BotInstance` to send message to telegram's chat
/// * `msg` - message to send
pub fn send_message(instance: &BotInstance, msg: &str) -> Result<(), ErrorResult> {
    let raw_url_str = format!("https://api.telegram.org/bot{bot_token}/sendMessage?chat_id={telegram_chat_id}&text={msg}", bot_token=instance.bot_token, telegram_chat_id=instance.chat_id, msg=msg);

    if let  Ok(url) = Url::parse(&raw_url_str) {
        match isahc::get(url.as_str()) {
            Ok(mut res) => {
                if res.status() == 200 {
                    // return success result
                    return Ok(());  // just return as unit
                }
                else {
                    // return error result
                    match res.json::<TelegramErrorResult>() {
                        Ok(json) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &json.description.to_owned()),
                        Err(_) => return utils::create_error_result_str(StatusCode::ErrorInternalError, "Error converting telegram error response to json")
                    }
                }
            },
            Err(e) => return utils::create_error_result_kind(StatusCode::ErrorInternalError, e.kind().clone()),
        }
    }

    utils::create_error_result_str(StatusCode::ErrorInternalError, "Error parsing internal telegram url")
}

/// Send message asynchronously.
/// Return `Result<(), ErrorResult>`.
///
/// # Arguments
///
/// * `instance` - `BotInstance` to send message to telegram's chat
/// * `msg` - message to send
pub async fn send_message_async(instance: &BotInstance, msg: &str) -> Result<(), ErrorResult> {
    let raw_url_str = format!("https://api.telegram.org/bot{bot_token}/sendMessage?chat_id={telegram_chat_id}&text={msg}", bot_token=instance.bot_token, telegram_chat_id=instance.chat_id, msg=msg);

    if let  Ok(url) = Url::parse(&raw_url_str) {
        match isahc::get_async(url.as_str()).await {
            Ok(mut res) => {
                if res.status() == 200 {
                    // return success result
                    return Ok(());  // just return as unit
                }
                else {
                    // return error result
                    match res.json::<TelegramErrorResult>().await {
                        Ok(json) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &json.description.to_owned()),
                        Err(_) => return utils::create_error_result_str(StatusCode::ErrorInternalError, "Error converting telegram error response to json")
                    }
                }
            },
            Err(e) => return utils::create_error_result_kind(StatusCode::ErrorInternalError, e.kind().clone()),
        }
    }

    utils::create_error_result_str(StatusCode::ErrorInternalError, "Error parsing internal telegram url")
}
