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
use teloxide::types::*;
use teloxide::prelude::*;

mod database;
mod handle;
use database as db;

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook2<'a>(bot: Bot) -> impl update_listeners::UpdateListener<Infallible> {
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

pub async fn webhook<'a>(bot: Bot) -> impl update_listeners::UpdateListener<Infallible> {
    // Heroku defines auto defines a port value
    let teloxide_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN env variable missing");
    let port: u16 = env::var("PORT")
        .expect("PORT env variable missing")
        .parse()
        .expect("PORT value to be integer");
    // Heroku host example .: "heroku-ping-pong-bot.herokuapp.com"
    let host = env::var("HOST").expect("have HOST env variable");
    let path = format!("bot{}", teloxide_token);
    let url = format!("https://{}/{}", host, path);

    bot.set_webhook(url)
        .send()
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::path(path))
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            let try_parse = match serde_json::from_str(&json.to_string()) {
                Ok(update) => Ok(update),
                Err(error) => {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                      This is a bug in teloxide, please open an issue here: \
                      https://github.com/teloxide/teloxide/issues.",
                        error,
                        json
                    );
                    Err(error)
                }
            };
            if let Ok(update) = try_parse {
                tx.send(Ok(update))
                    .expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let serve = warp::serve(server);

    let address = format!("0.0.0.0:{}", port);
    tokio::spawn(serve.run(address.parse::<SocketAddr>().unwrap()));
    rx
}

async fn handle_callback(cx: UpdateWithCx<CallbackQuery>) {
    log::debug!("archive handle_callback unwrite func");
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("start");

    let bot = Bot::from_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable missing");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Cannot connect to database");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::info!("Database connection error: {}", e);
        }
    });

    match db::DB.set(client) {
        Ok(_) => log::info!("Database connected"),
        _ => log::info!("Something wrong with database"),
    }

    Dispatcher::new(bot.clone())
        .inline_queries_handler(|rx: DispatcherHandlerRx<InlineQuery>| {
            rx.for_each(|message| async move {
                let photo = [
                    "AgADAQADQakxG03uEUXL8miDe_Bu1mkHc0oXAAMBAAMCAAN5AAMuPAACFgQ",
                ];
                message
                    .bot
                    .answer_inline_query(
                        message.update.id.clone(),
                        photo
                            .iter()
                            .enumerate()
                            .map(|(i, x)| {
                                let mut cp = InlineQueryResultCachedPhoto::new(
                                    "id", 
                                    (*x).to_string()
                                );
                                //cp.title("val");
                                // InlineQueryResult::CachedSticker(
                                //     InlineQueryResultCachedSticker::new(
                                //        format!("5364356453645365345633456346543{}", i),  
                                //        (*x).to_string()
                                // ))
                                
                                cp = cp.title("fubuki!");
                                InlineQueryResult::CachedPhoto(
                                    cp
                                )
                            })
                            .collect::<Vec<InlineQueryResult>>(),
                    )
                    .send()
                    .await
                    .log_on_error()
                    .await;
                log::info!("inline: {:?}", message.update);
            })
        })
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            log::debug!("archive messages_handler");
            rx.for_each_concurrent(None, |message| async move {
                handle::handle_message(message)
                    .await
                    .expect("Something wrong with the bot!");
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<CallbackQuery>| {
            log::debug!("archive callback_queries_handler A");
            rx.for_each_concurrent(None, |cx| async move { handle_callback(cx).await })
        })
        .dispatch_with_listener(
            webhook(bot).await,
            LoggingErrorHandler::with_custom_text(""),
        )
        .await;
}
