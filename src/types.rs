use isahc::prelude::*;

/// Requests will be sent according to bot instance.
/// So users can use this library interacting with multiple bot APIs by having
/// multiple of `BotInstance`.
///
/// Most of the exposed function require `BotInstance` as part of the arguments.
#[derive(Clone)]
pub struct BotInstance {
    pub bot_token: String,
    pub chat_id: String,
}

/// ErrorResult usually returned to indicate result from calling APIs related
/// functions.
pub struct ErrorResult {
    pub code: u16,       // error returned code
    pub msg: String,     // error string description
}

/// Telegram's error result.
/// In case of error occurred as part of telegram API calling, then this struct
/// will be formed and returned.
#[derive(Debug, serde::Deserialize)]
pub struct TelegramErrorResult {
    pub ok: bool,
    pub error_code: i32,
    pub description: String,
}

/// Status code indicating the result of APIs related function call.
pub enum StatusCode {
    /// Success
    Success=0,

    /// Internal error due to various internal operations.
    /// Whenever Telegram's related operations occurred with error, then this
    /// value will be used.
    ErrorInternalError,
}
