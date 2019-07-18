use telegram_bot::{CanReplySendMessage, MessageKind, UpdateKind};
use std::cell::RefCell;
use std::rc::Rc;

use super::config;

pub fn handle(update:telegram_bot::Update, api:Rc<RefCell<telegram_bot::Api>>) -> Result<(),telegram_bot::Error>{
    if let UpdateKind::ChannelPost(message) = update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                //println!("<{}>: {}", &message.from.first_name, data);

                let mut matches:Vec<_> = config::RE.matches(data).into_iter().collect::<Vec<usize>>();
                let mut ret = "".to_string();  

                if  matches.len() == 0 {
                    return Ok(())
                }      

                matches.reserve(matches.len());

                let mut i = 0usize;
                for j in &config::CONF.auto_resp {
                    if &i == matches.get(0usize).unwrap_or(&0usize) {
                        ret += &j.value[0];
                        matches.pop();
                    }
                    i = i + 1;
                }
                
                // for i in &config::CONF.auto_resp {
                //     match caps.name(&i.key) {
                //         Some(_)=>ret += &i.value[0].clone(),
                //         _ => ret += "what are you saying",
                //     }
                // }
                // Answer message with "Hi".
                api.borrow_mut().spawn(message.text_reply(
                    format!("{}", ret)
                ));
        }
    }

    Ok(())
}