use teloxide::{utils::command::BotCommand, prelude::*};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
}

pub async fn handle_message(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    match cx.update.text() {
        None => {
            log::debug!("archive messages_handler");
            cx.answer_str("text").await
        },
        Some(text) => {
            log::debug!("archive command");
            if let Ok(command) = Command::parse(text, "") {
                match command {
                    Command::Help => cx.answer_str(Command::descriptions()).await,
                }
            } else {
                cx.reply_to("balabalabala")
                .send()
                .await
            }
        }
    }
}
