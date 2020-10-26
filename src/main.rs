use rand::Rng;
use reqwest::StatusCode;
use std::time::Duration;
use std::{convert::Infallible, env, net::SocketAddr};
use teloxide::{
    dispatching::update_listeners,
    prelude::*,
    types::{
        CallbackQuery, ChatId, ChatOrInlineMessage, InlineKeyboardButton, InlineKeyboardMarkup,
    },
    utils::command::BotCommand,
};
use tokio::{sync::mpsc, time::delay_for};
use tokio_postgres::NoTls;
use warp::Filter;

mod handle;

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
                tx.send(Ok(update))
                    .expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let serve = warp::serve(server);

    // You might want to use serve.key_path/serve.cert_path methods here to
    // setup a self-signed TLS certificate.

    tokio::spawn(serve.run("0.0.0.0:80".parse::<SocketAddr>().unwrap()));
    rx
}

async fn handle_callback(cx: UpdateWithCx<CallbackQuery>) {}

async fn run() {
    teloxide::enable_logging!();
    log::info!("start");

    let bot = Bot::from_env();

    Dispatcher::new(bot.clone())
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, |message| async move {
                handle::handle_message(message)
                    .await
                    .expect("Something wrong with the bot!");
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<CallbackQuery>| {
            rx.for_each_concurrent(None, |cx| async move { handle_callback(cx).await })
        })
        .dispatch_with_listener(
            webhook(bot).await,
            LoggingErrorHandler::with_custom_text(""),
        )
        .await;
}
