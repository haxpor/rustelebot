#[cfg(test)]
mod test {
    use crate::*;   // to import lib.rs
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

        if let Err(_) = send_message(&instance, "test") {
            assert!(false);
        }
    }

    /// Each messasge will be sent one by one in order.
    /// Usually message will be received in order on telegram, but in rare case
    /// that telegram server cannot process the incoming message in time, then
    /// it might come in not in order.
    #[test]
    fn test_send_messages_sync() {
        let instance = get_instance();

        if let Err(_) = send_message(&instance, "[Rustelebot] msg1") {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg2") {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg3") {
            assert!(false)
        }
        if let Err(_) = send_message(&instance, "[Rustelebot] msg4") {
            assert!(false)
        }
    }

    /// Order of message received on telegram is not guaranteed.
    #[test]
    fn test_send_messages_async() {
        let instance = get_instance();

        async fn async_fn(instance: &BotInstance) {
            let f1 = send_message_async(&instance, "[Rustelebot] msg1-async");
            let f2 = send_message_async(&instance, "[Rustelebot] msg2-async");
            let f3 = send_message_async(&instance, "[Rustelebot] msg3-async");
            let f4 = send_message_async(&instance, "[Rustelebot] msg4-async");

            // wait for all futures
            // this doesn't not guarantee order
            let _ = futures::join!(f1, f2, f3, f4);
        }

        // block on the current thread for the whole async (futures) completes
        futures::executor::block_on(async_fn(&instance));
    }
}
