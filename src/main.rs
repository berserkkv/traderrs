mod tools;
mod calculator;
mod enums;
mod ta;
mod position_manager;
mod logger;
mod bot_manager;
mod binance_connector;
mod strategy;
mod models;
mod api;

use std::sync::Arc;
use axum::{
    http::StatusCode,
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use mime_guess::MimeGuess;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::binance_connector::BinanceConnector;
use crate::bot_manager::BotManager;
use crate::enums::Symbol::SolUsdt;
use crate::enums::Timeframe::Min1;
use crate::logger::init_logger;
use crate::models::bot::Bot;
use crate::models::models::ManagerChannel;
use crate::position_manager::PositionManager;

#[derive(RustEmbed)]
#[folder = "ui/build"]
struct Assets;

impl Assets {
    fn get_file(path: &str) -> Option<(Vec<u8>, String)> {
        Assets::get(path).map(|file| {
            let mime = MimeGuess::from_path(path)
                .first_or_octet_stream()
                .as_ref()
                .to_owned();
            (file.data.into_owned(), mime)
        })
    }

    fn router() -> Router {
        let mut router = Router::new();

        for file_path in Assets::iter() {
            let path_string = file_path.as_ref().to_string();

            // prepend `/` because Axum expects routes to start with '/'
            let route_path = format!("/{}", path_string);

            // clone for closure
            let path_for_closure = path_string.clone();

            let handler = get(move || {
                let path_for_closure = path_for_closure.clone();
                async move {
                    if let Some((content, mime)) = Assets::get_file(&path_for_closure) {
                        (StatusCode::OK, [("Content-Type", mime)], content)
                    } else {
                        (StatusCode::NOT_FOUND, [("Content-Type", "text/plain".to_string())], Vec::new())
                    }
                }
            });

            router = router.route(&route_path, handler.clone());

            // Handle /somepath/ route if the file is /somepath/index.html
            if path_string.ends_with("/index.html") {
                let prefix = path_string.strip_suffix("index.html").unwrap();
                let prefix_route = format!("/{}", prefix);
                router = router.route(&prefix_route, handler.clone());
            }
        }

        router
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {

    init_logger();

    let bots = init_bots();

    let ch = ManagerChannel { for_bot_manager: vec![], for_position_manager: vec![] };
    let channel = Arc::new(Mutex::new(ch));


    let connector = BinanceConnector::new();
    let mut position_manager = PositionManager::new(connector.clone(), Arc::clone(&channel));
    let mut bot_manager = BotManager::new(bots, connector, Arc::clone(&channel));


    tokio::spawn(async move {
        loop {
            position_manager.monitor().await;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await; // adjust duration as needed
        }
    });

    tokio::spawn(async move {
        bot_manager.start().await;
    });

    // Static assets router
    let assets_router = Assets::router();

    // Fallback route: serve index.html for any other path (for SPA)
    let fallback = get(|| async {
        if let Some((content, mime)) = Assets::get_file("index.html") {
            (StatusCode::OK, [("Content-Type", mime)], content)
        } else {
            (StatusCode::NOT_FOUND, [("Content-Type", "text/plain".to_string())], "index.html not found".into())
        }
    });


    // Combine router with fallback
    let app = assets_router.fallback(fallback);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}




fn init_bots() -> Vec<Bot> {
    let mut bots = Vec::new();
    let capital = 100.0;
    let leverage = 10.0;
    let take_profit_ratio = 0.8;
    let stop_loss_ratio = 0.4;
    let trailing_stop_activation_point = 0.1;

    let tf = [Min1];
    let st = ["EmaMacd", "EmaMacd2"];
    let smb = [SolUsdt];


    for t in tf.iter() {
        for s in st.iter() {
            for symbol in smb.iter() {
                let bot = Bot::new(
                    *t,
                    *symbol,
                    s.to_string(),
                    capital,
                    leverage,
                    take_profit_ratio,
                    stop_loss_ratio,
                    trailing_stop_activation_point,
                );
                bots.push(bot);
            }
        }
    }

    bots
}

