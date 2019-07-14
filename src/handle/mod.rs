use telegram_bot::*;
use futures::*;
use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;

use super::config;

pub fn handle(update:telegram_bot::Update, api:Rc<RefCell<telegram_bot::Api>>) -> Result<(),telegram_bot::Error>{
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                let conf : &config::Config = &config::CONF;
                for i in &conf.auto_resp {
                    println!("{}", i.key);
                }
                let re = Regex::new(r"").unwrap();
                re.captures(data).unwrap();
                // Answer message with "Hi".
                api.borrow_mut().spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
        }
    }

    Ok(())
}