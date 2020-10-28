use teloxide::{utils::command::BotCommand, prelude::*};
use super::database as db;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "仅仅支持这些命令：")]
enum Command {
    #[command(description = "显示帮助.")]
    Help,
    #[command(description = "发一张黄图.")]
    Erotic,
    #[command(description = "记忆一张黄图.")]
    AddErotic(String),
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
            match Command::parse(text, "") {
                Ok(command) => {
                    match command {
                        Command::Help => cx.answer_str(Command::descriptions()).await,
                        Command::Erotic => {
                            if let Some(ret) = db::query_erotic().await{
                                //"https://twitter.com/dengeki_maoh/status/1320943775590436865?s=20"
                                cx.answer_str(ret).await
                            } else {
                                cx.answer_str("没有存货").await
                            }
                            
                        },
                        Command::AddErotic(url) => {
                            cx.answer_str("Ok").await
                        },
                    }
                }
                Err(_) => {
                            cx.reply_to("别烦我")
                            .send()
                            .await
                        }
            }
        }
    }
}
