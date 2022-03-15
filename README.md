# rustelebot
Telegram Bot API in Rust. Right now it supports only send message with [sendMessage](https://core.telegram.org/bots/api#sendmessage)
with no exhuastive features or input fields of such API.

Main use case of this library is to integrate it with your application
to quickly send message to telegram bot, in turn send message to telegram chat group
or telegram channel group provided that we know `chat_id`.

# APIs

* `create_instance` - create a bot instance consistsing of Telegram's bot token, and target `chat_id`
* `send_message` - call Telegram bot's API `sendMessage` to send message synchronously
* `send_message_async` - call Telegram bot's API `sendMessage` to send message asynchronously

# Example

## Send Synchronously

### Send in simple way

```rust
fn main() {
	let instance = rustelebot::create_instance("123456:123456", "-1000000");
	if let Err(_) = rustelebot::send_message(&instance, "Hello world", None) {
		// error handling here...
	}
}
```

### Send in `MarkdownV2` or `HTML` style message

Send message in `MarkdownV2`

```rust
use rustelebot::{SendMessageOption, SendMessageParseMode};

fn main() {
	let instance = rustelebot::create_instance("123456:123456", "-1000000");
	let option = SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) };

	// note on two spaces at the end of the line for a new line in markdown
	if let Err(_) = rustelebot::send_message(&instance,
r#"__Hello world__  
`Tap to copy this text`  
Visit my [website](https://wasin.io)"#, Some(option)) {
		// error handling here...
	}
}
```

Send messsage in `HTML`

```rust
use rustelebot::{SendMessageOption, SendMessageParseMode};

fn main() {
	let instance = rustelebot::create_instance("123456:123456", "-1000000");
	let option = SendMessageOption { parse_mode: Some(SendMessageParseMode::HTML) };

	if let Err(_) = rustelebot::send_message(&instance,
r#"<u>Hello world</u>
<code>Tap to copy this text</code>
Visit my <a href="https://wasin.io">website</a>"#, Some(option)) {
		// error handling here...
	}
}
```

# Send Asynchronously

```rust
fn main() {
	let instance = rustelebot::create_instance("123456:123456", "-1000000");

	async fn async_fn(instance: &BotInstance) {
		let f1 = rustelebot::send_message_async(&instance, "Msg1", None);
		let f2 = rustelebot::send_message_async(&instance, "Msg2", None);
		let f3 = rustelebot::send_message_async(&instance, "Msg3", None);
		let f4 = rustelebot::send_message_async(&instance, "Msg4", None);

		// wait for all futures
		// this doesn't not guarantee order
		futures::join!(f1, f2, f3, f4);
	}

	// block on the current thread for the whole async (futures) to complete
	futures::executor::block_on(async_fn(&instance));
}
```

# Tests

You can test by define the following two environment variables

* `RUSTELEBOT_BOT_TOKEN` - telegram bot's token
* `RUSTELEBOT_CHAT_ID` - telegram bot's chat id

then execute

`cargo test`

some tests will send a single, or multiple messages to specified chat id on behalf
of such telegram bot. Please take a look at `src/tests.rs`.

# Note

You can utilize this telegram bot `@username_to_id_bot` in order to get your
telegram channel's `chat_id`.

# License
MIT, Wasin Thonkaew
