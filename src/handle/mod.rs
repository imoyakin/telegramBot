use std::cell::RefCell;
use std::rc::Rc;
use telegram_bot::{CanReplySendMessage, MessageKind, UpdateKind};
use rand::prelude::*;

use super::config;

pub fn handle(
    update: telegram_bot::Update,
    api: Rc<RefCell<telegram_bot::Api>>,
) -> Result<(), telegram_bot::Error> {
    match update.kind {
        UpdateKind::ChannelPost(post) => {
            if let MessageKind::Text{ref data, ..} = post.kind {
                println!{"{}", data};
            };
        }
        UpdateKind::Message(message) => {
            if let MessageKind::Text { ref data, ref entities} = message.kind {
                // Print received text message to stdout.
                //println!("<{}>: {}", &message.from.first_name, data);
                
                for x in entities.iter() {
                    match x {
                        Mention => println!("has been metion"),
                        BotCommand => println!("bot BotCommand"),
                        _ => println!("unkown"),
                    };
                }

                let mut matches: Vec<_> =
                    config::RE.matches(data).into_iter().collect::<Vec<usize>>();
                let mut ret = "".to_string();

                if matches.len() == 0 {
                    return Ok(());
                }

                matches.reserve(matches.len());

                let mut i = 0usize;
                for j in &config::CONF.auto_resp {
                    if &i == matches.get(0usize).unwrap_or(&0usize) {
                        let mut rng = thread_rng();
                        let roll = rng.gen_range(0, j.num);
                        ret += &j.value[roll];
                        matches.pop();
                    }
                    i = i + 1;
                }
                api.borrow_mut()
                    .spawn(message.text_reply(format!("{}", ret)));
            }
        }
        UpdateKind::CallbackQuery(callback) => {
            println!{"{}", callback.data}
            println!{"{}", callback.chat_instance}
        }
        _ => {}
    };

    Ok(())
}
