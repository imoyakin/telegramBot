use telegram_bot::*;
use futures::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn handle(update:telegram_bot::Update, api:Rc<RefCell<telegram_bot::Api>>) -> Result<(),telegram_bot::Error>{
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);
                // Answer message with "Hi".
                api.borrow_mut().spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
        }
    }

    Ok(())
}