mod binance_connector;
mod calculator;
mod entry_manager;
mod enums;
mod logger;
mod models;
mod position_manager;
mod strategy;
mod ta;
mod tools;

use crate::binance_connector::BinanceConnector;
use crate::entry_manager::EntryManager;
use crate::enums::Symbol::{BnbUsdt, BtcUsdt, EthUsdt, SolUsdt};
use crate::enums::Timeframe::{Min1, Min15, Min5};
use crate::logger::init_logger;
use crate::models::bot::Bot;
use crate::models::models::{Order, SystemInfo};
use crate::position_manager::PositionManager;
use axum::extract::Path;
use axum::routing::put;
use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use chrono::{DateTime, FixedOffset, Utc};
use log::info;
use mime_guess::MimeGuess;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::System;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

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
            let route_path = format!("/{}", path_string);

            let path_for_closure = path_string.clone();

            let handler = get(move || {
                let path_for_closure = path_for_closure.clone();
                async move {
                    if let Some((content, mime)) = Assets::get_file(&path_for_closure) {
                        (StatusCode::OK, [("Content-Type", mime)], content)
                    } else {
                        (
                            StatusCode::NOT_FOUND,
                            [("Content-Type", "text/plain".to_string())],
                            Vec::new(),
                        )
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

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    init_logger();

    let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
    let started_time = Utc::now().with_timezone(&offset);

    let (bots, order_map) = init_dependencies();

    // Static assets router
    let assets_router = Assets::router();

    let fallback = get(|| async {
        if let Some((content, mime)) = Assets::get_file("index.html") {
            (StatusCode::OK, [("Content-Type", mime)], content)
        } else {
            (
                StatusCode::NOT_FOUND,
                [("Content-Type", "text/plain".to_string())],
                "index.html not found".into(),
            )
        }
    });

    let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);

    let app = assets_router
      .route("/api/v1/bots", get(get_all_bot))
      .route("/api/v1/bots/{id}/orders", get(get_orders_by_id))
      .route("/api/v1/bots/reset", put(reset_bots))
      .route("/api/v1/system", get(get_system_usage))
      .layer(Extension(bots))
      .layer(Extension(order_map))
      .layer(Extension(started_time))
      .layer(cors)
      .fallback(fallback);

    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    info!("listening on https://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_bot(Extension(bots): Extension<Arc<Vec<RwLock<Bot>>>>) -> Json<Vec<Bot>> {
    let mut v = Vec::with_capacity(bots.len());
    for b in bots.iter() {
        v.push(b.read().await.clone());
    }

    tools::sort_bots(&mut v);
    Json(v)
}

async fn get_orders_by_id(Path(id): Path<i64>, Extension(order_map): Extension<Arc<RwLock<HashMap<i64, Vec<Order>>>>>) -> Json<Vec<Order>> {
    // let mut orders: Vec<Order> = Vec::new();
    //
    // for _ in 0..id {
    //     orders.push(Order::dummy());
    // }
    //
    // let  x = 1.0;
    //
    // for i in 0..orders.len() {
    //     if i % 2 == 0 {
    //         orders[i].pnl += x;
    //     } else {
    //         orders[i].pnl -= x / 2.0;
    //     }
    // }
    //
    // Json(orders)

    let mut orders = order_map
      .read()
      .await
      .get(&id)
      .cloned()
      .unwrap_or(Vec::new());
    orders.reverse();

    Json(orders)
}

async fn reset_bots(Extension(bots): Extension<Arc<Vec<RwLock<Bot>>>>){
    for b in bots.iter() {
        b.write().await.reset();
    }
}

async fn get_system_usage(Extension(started_time): Extension<DateTime<FixedOffset>>) -> Json<SystemInfo> {
    let sys = System::new_all();
    let mut cpu_usage: f32 = 0.0;

    for (_, cpu) in sys.cpus().iter().enumerate() {
        cpu_usage += cpu.cpu_usage();
    }

    let total = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = used_memory * 100 / total;
    Json(SystemInfo {
        cpu_usage,
        memory_usage,
        started_time,
    })
}

fn init_dependencies() -> (Arc<Vec<RwLock<Bot>>>, Arc<RwLock<HashMap<i64, Vec<Order>>>>) {
    let bots = Arc::new(init_bots());

    let connector = BinanceConnector::new();
    let orders_map: Arc<RwLock<HashMap<i64, Vec<Order>>>> = Arc::new(RwLock::new(HashMap::new()));
    let mut position_manager = PositionManager::new(
        Arc::clone(&bots),
        Arc::new(connector.clone()),
        Arc::clone(&orders_map),
    );
    let mut entry_manager = EntryManager::new(Arc::clone(&bots), Arc::new(connector));

    tokio::spawn(async move {
        position_manager.start().await;
    });

    tokio::spawn(async move {
        entry_manager.start().await;
    });

    (bots, orders_map)
}

fn init_bots() -> Vec<RwLock<Bot>> {
    let mut bots = Vec::new();
    let capital = 100.0;
    let leverage = 10.0;
    let take_profit_ratio = 0.8;
    let stop_loss_ratio = 0.4;
    let trailing_stop_activation_point = 0.1;

    let tf = [Min1, Min5, Min15];
    let st = ["EmaMacd", "EmaMacd2", "EmaBounce"];
    let smb = [SolUsdt, EthUsdt, BnbUsdt, BtcUsdt];

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
                bots.push(RwLock::new(bot));
            }
        }
    }

    bots
}
