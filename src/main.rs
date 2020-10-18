use teloxide::{dispatching::update_listeners, prelude::*};
	
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::mpsc;
use warp::Filter;

use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook<'a>(bot: Bot) -> impl update_listeners::UpdateListener<Infallible> {
    // You might want to specify a self-signed certificate via .certificate
    // method on SetWebhook.
    bot.set_webhook("Your HTTPS ngrok URL here. Get it by 'ngrok http 80'")
        .send()
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            if let Ok(update) = Update::try_parse(&json) {
                tx.send(Ok(update)).expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let serve = warp::serve(server);

    // You might want to use serve.key_path/serve.cert_path methods here to
    // setup a self-signed TLS certificate.

    tokio::spawn(serve.run("127.0.0.1:80".parse::<SocketAddr>().unwrap()));
    rx
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("start");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;

    // let args: Vec<String> = env::args().collect();
    // // println!("{}",config::conf.AutoResp.Key);

    // let mut core = Core::new().unwrap();

    // let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    // let api = Api::configure(token).build(core.handle()).unwrap();

    // let api_call = Rc::new(RefCell::new(api));

    // // Fetch new updates via long poll method
    // let future = api_call.try_borrow().unwrap().stream().for_each(|update|{
    //     handle::handle(update,api_call.clone())
    // });

    // core.run(future).unwrap();
}