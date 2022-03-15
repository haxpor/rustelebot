use isahc::prelude::*;
use url::Url;
use types::*;

#[cfg(test)]
pub mod tests;

pub mod types;
pub mod utils;
pub mod isahc_utils;
mod impls;

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
/// * `options` - options for sending message
pub fn send_message(instance: &BotInstance, msg: &str, options: Option<SendMessageOption>) -> Result<(), ErrorResult> {
    let raw_url_str = format!("https://api.telegram.org/bot{}/sendMessage", instance.bot_token);
    let url = match Url::parse(&raw_url_str) {
		Ok(res) => res,
        Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error parsing Url; err={}", e)),
	};

	// declare a request struct used only in this function scope
    // NOTE: serde::Serialize can work with &str
    #[derive(Debug, serde::Serialize)]
    struct RequestObj<'a> {
        chat_id: &'a str,
        text: &'a str,

        // this is required unfortunately, see https://github.com/serde-rs/serde/issues/947
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<&'a str>,
    }

    // create a request object which contains parameters needed for the API
    let request_json_obj = RequestObj {
        chat_id: &instance.chat_id,
        text: msg,
        parse_mode: if options.is_some() && options.as_ref().unwrap().parse_mode.is_some() { Some(utils::get_send_message_parse_mode_str(options.unwrap().parse_mode.unwrap())) } else { None }
    };

    // serialize as json byte vector
    let request_json_obj_body = match serde_json::to_vec(&request_json_obj) {
        Ok(res) => res,
        Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error serializing HTTP request object into JSON byte vector; err={}", e)),
    };

    // telegram supports both GET, and POST with various content-type
    // 'application/json' is one of them that telegram supports
    let request = match isahc::Request::builder()
        .method("POST")
        .uri(url.as_str())
        .header("content-type", "application/json")
        .version_negotiation(isahc::config::VersionNegotiation::http2())
        .body(request_json_obj_body) {
            Ok(res) => res,
            Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error building HTTP request; err={}", e)),
    };

	match isahc::send(request) {
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

/// Send message asynchronously.
/// Return `Result<(), ErrorResult>`.
///
/// # Arguments
///
/// * `instance` - `BotInstance` to send message to telegram's chat
/// * `msg` - message to send
/// * `options` - options for sending message
pub async fn send_message_async(instance: &BotInstance, msg: &str, options: Option<SendMessageOption>) -> Result<(), ErrorResult> {
    let raw_url_str = format!("https://api.telegram.org/bot{}/sendMessage", instance.bot_token);
    let url = match Url::parse(&raw_url_str) {
		Ok(res) => res,
        Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error parsing Url; err={}", e)),
	};

	// declare a request struct used only in this function scope
    // NOTE: serde::Serialize can work with &str
    #[derive(Debug, serde::Serialize)]
    struct RequestObj<'a> {
        chat_id: &'a str,
        text: &'a str,

        // this is required unfortunately, see https://github.com/serde-rs/serde/issues/947
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<&'a str>,
    }

    // create a request object which contains parameters needed for the API
    let request_json_obj = RequestObj {
        chat_id: &instance.chat_id,
        text: msg,
        parse_mode: if options.is_some() && options.as_ref().unwrap().parse_mode.is_some() { Some(utils::get_send_message_parse_mode_str(options.unwrap().parse_mode.unwrap())) } else { None }
    };

    // serialize as json byte vector
    let request_json_obj_body = match serde_json::to_vec(&request_json_obj) {
        Ok(res) => res,
        Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error serializing HTTP request object into JSON byte vector; err={}", e)),
    };

    // telegram supports both GET, and POST with various content-type
    // 'application/json' is one of them that telegram supports
    let request = match isahc::Request::builder()
        .method("POST")
        .uri(url.as_str())
        .header("content-type", "application/json")
        .version_negotiation(isahc::config::VersionNegotiation::http2())
        .body(request_json_obj_body) {
            Ok(res) => res,
            Err(e) => return utils::create_error_result_str(StatusCode::ErrorInternalError, &format!("Error building HTTP request; err={}", e)),
    };

    match isahc::send_async(request).await {
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
