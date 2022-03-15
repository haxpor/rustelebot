#[cfg(test)]
mod test {
    use crate::*;   // to import lib.rs
    use crate::types::{SendMessageOption, SendMessageParseMode};
    use std::env;

    /// Reading bot token, and chat id for inteacting with telegram bot.
    /// Return tuple of (bot token, and telegrama's chat id) if successfully read,
    /// otherwise return a tuple of success flag. Both returned tuples are returned
    /// in order of bot token, and chat id.
    ///
    /// Set environment variable as follows before running test
    /// `RUSTELEBOT_BOT_TOKEN` - telegram bot's token
    /// `RUSTELEBOT_CHAT_ID` - telegram bot's chat id
    ///
    /// These environment variable names are used for testing purpose only.
    fn get_keys() -> Result<(String, String), (bool, bool)> {
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        let mut results = (false, false);

        if let Ok(st) = env::var("RUSTELEBOT_BOT_TOKEN") {
            keys.0 = st;
            results.0 = true;
        }

        if let Ok(st) = env::var("RUSTELEBOT_CHAT_ID") {
            keys.1 = st;
            results.1 = true;
        }

        if results.0 && results.1 {
            Ok(keys)
        }
        else {
            Err(results)
        }
    }

    /// Get a proper error message from the tuple of success flag
    /// for getting value from environment variables.
    /// See `get_keys()`.
    fn get_ret_error_msg(errors: (bool, bool)) -> &'static str {
        if !errors.0 && !errors.1 {
            return "RUSTELEBOT_BOT_TOKEN and RUSTELEBOT_CHAT_ID environment variable are not set";
        }
        if !errors.0 {
            return "RUSTELEBOT_BOT_TOKEN environment variable is not set";
        }
        return "RUSTELEBOT_CHAT_ID environment variable is not set";
    }

    fn get_instance() -> BotInstance {
        let mut keys: (String, String) = ("".to_string(), "".to_string());
        match get_keys() {
            Ok(ret_keys) => keys = ret_keys,
            Err(errs) => assert!(false, "{}", get_ret_error_msg(errs))
        }

        create_instance(&keys.0.to_owned(), &keys.1.to_owned())
    }

    #[test]
    fn test_send_message_simple() {
        let instance = get_instance();

        if let Err(_) = send_message(&instance, "test", None) {
            assert!(false);
        }
    }

    #[test]
    fn test_send_markdown_style_message() {
        let instance = get_instance();

        // NOTE: 2 spaces at the end of the line in Markdown to include a newline
        match send_message(&instance,
r#"\[Rustelebot\] __MarkdownV2__  
`Tap to copy this text`\.  
You can visit my [website](https://wasin.io)\.  
Woot\!"#,
                            Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) })) {
            Err(e) => panic!("error sending markdown style message; err={}", e),
            _ => (),
        }
    }

    #[test]
    fn test_send_markdown_style_message_async() {
        let instance = get_instance();

        async fn intern_fn(instance: &BotInstance) {
            let f1 = send_message_async(&instance,
r#"\[Rustelebot\] __MarkdownV2__  
*async msg 1*  
`Tap to copy this text`\.  
You can visit my [website](https://wasin.io)\.  
Woot\!"#,
                                        Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) }));

            let f2 = send_message_async(&instance,
r#"\[Rustelebot\] __MarkdownV2__  
*async msg 2*  
`Tap to copy this text`\.  
You can visit my [website](https://wasin.io)\.  
Woot\!"#,
                                        Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) }));

            let (res1, res2) = futures::join!(f1, f2);
            if res1.is_err() {
                panic!("f1 has error");
            }
            if res2.is_err() {
                panic!("f2 has error");
            }
        }

        futures::executor::block_on(intern_fn(&instance));
    }

    /// Send HTML style message
    #[test]
    fn test_send_html_style_message() {
        let instance = get_instance();

        match send_message(&instance,
r#"[Rustelebot] <u>HTML style</u>
<code>Tap to copy this text</code>.
You can visit my <a href="https://wasin.io">website</a>.
Woot!"#,
                            Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::HTML) })) {
            Err(e) => panic!("error sending html style message; err={}", e),
            _ => (),
        }
    }

    /// Send HTML style message (async)
    #[test]
    fn test_send_html_style_message_async() {
        let instance = get_instance();

        async fn intern_fn(instance: &BotInstance) {
            let f1 = send_message_async(&instance,
r#"[Rustelebot] <u>HTML style</u> - <b>async msg 1</b>
<code>Tap to copy this text</code>.
You can visit my <a href="https://wasin.io">website</a>.
Woot!"#,
                                        Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::HTML) }));

            let f2 = send_message_async(&instance,
r#"[Rustelebot] <u>HTML style</u> - <b>async msg 2</b>
<code>Tap to copy this text</code>.
You can visit my <a href="https://wasin.io">website</a>.
Woot!"#,
                                        Some(SendMessageOption { parse_mode: Some(SendMessageParseMode::HTML) }));

            let (res1, res2) = futures::join!(f1, f2);
            if res1.is_err() {
                panic!("f1 has error");
            }
            if res2.is_err() {
                panic!("f2 has error");
            }
        }

        futures::executor::block_on(intern_fn(&instance));
    }

    /// Each messasge will be sent one by one in order.
    /// Usually message will be received in order on telegram, but in rare case
    /// that telegram server cannot process the incoming message in time, then
    /// it might come in not in order.
    #[test]
    fn test_send_messages_sync() {
        let instance = get_instance();

        if let Err(_) = send_message(&instance, "[Rustelebot] msg1", None) {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg2", None) {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg3", None) {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg4", None) {
            assert!(false)
        }
    }

    /// Order of message received on telegram is not guaranteed.
    #[test]
    fn test_send_messages_async() {
        let instance = get_instance();

        async fn async_fn(instance: &BotInstance) {
            let f1 = send_message_async(&instance, "[Rustelebot] msg1-async", None);
            let f2 = send_message_async(&instance, "[Rustelebot] msg2-async", None);
            let f3 = send_message_async(&instance, "[Rustelebot] msg3-async", None);
            let f4 = send_message_async(&instance, "[Rustelebot] msg4-async", None);

            // wait for all futures
            // this doesn't not guarantee order
            let (res1, res2, res3, res4) = futures::join!(f1, f2, f3, f4);
            if res1.is_err() {
                panic!("f1 has error");
            }
            if res2.is_err() {
                panic!("f2 has error");
            }
            if res3.is_err() {
                panic!("f3 has error");
            }
            if res4.is_err() {
                panic!("f4 has error");
            }
        }

        // block on the current thread for the whole async (futures) completes
        futures::executor::block_on(async_fn(&instance));
    }
}
