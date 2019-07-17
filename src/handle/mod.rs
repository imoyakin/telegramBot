use telegram_bot::{CanReplySendMessage, MessageKind, UpdateKind};
use std::cell::RefCell;
use std::rc::Rc;

use super::config;

pub fn handle(update:telegram_bot::Update, api:Rc<RefCell<telegram_bot::Api>>) -> Result<(),telegram_bot::Error>{
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                let caps = config::RE.captures(data).unwrap();
                let mut ret = "".to_string();
                for i in &config::CONF.auto_resp {
                    match caps.name(&i.key) {
                        Some(_)=>ret += &i.value[0].clone(),
                        _ => ret += "what are you saying",
                    }
                }
                // Answer message with "Hi".
                api.borrow_mut().spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, ret)
                ));
        }
    }

    Ok(())
}