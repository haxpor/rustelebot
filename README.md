# rustelebot
Telegram Bot API in Rust. Right now it supports only send message with [sendMessage](https://core.telegram.org/bots/api#sendmessage)
with no exhuastive features or input fields of such API.

Main use case of this library is to integrate it with your application
to quickly send message to telegram bot, in turn send message to telegram chat group
or telegram channel group provided that we know `chat_id`.

# APIs

* `pub fn send_message(instance: &BotInstance, msg: &str) -> Result<(), ErrorResult>`
* `pub async fn send_message_async(instance: &BotInstance, msg: &str) -> Result<(), ErrorResult>`

# Tests

You can test by define the following two environment variables

* `RUSTELEBOT_BOT_TOKEN` - telegram bot's token
* `RUSTELEBOT_CHAT_ID` - telegram bot's chat id

then execute

`cargo test`

some tests will send a single, or multiple messages to specified chat id on behalf
of such telegram bot.

# License
MIT, Wasin Thonkaew
