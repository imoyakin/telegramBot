extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate lazy_static;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::rc::Rc;

mod config;
mod handle;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}",config::conf.AutoResp.Key);

    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    let api_call = Rc::new(RefCell::new(api));

    // Fetch new updates via long poll method
    let future = api_call.try_borrow().unwrap().stream().for_each(|update|{
        handle::handle(update,api_call.clone())
    });

    core.run(future).unwrap();
}