use crate::Configuration;
use futures::StreamExt;
use telegram_bot::*;

// For now, use sample code from github.com/telegram-rs/telegram-bot as a test...
#[tokio::main]
pub(crate) async fn run(args: &Configuration) -> Result<(), Error> {
    let api = Api::new(&args.bot_token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                ))).await?;
            }
        }
    }
    return Ok(())
}