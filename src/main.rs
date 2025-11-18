mod api;
mod calculator;
mod connector;
mod entry_manager;
mod enums;
mod logger;
mod models;
mod position_manager;
mod repository;
mod strategy;
mod ta;
mod tools;

use crate::api::get_router;
use crate::connector::BinanceConnector;
use crate::entry_manager::EntryManager;
use crate::enums::Symbol::{BnbUsdt, BtcUsdt, EthUsdt, SolUsdt};
use crate::enums::Timeframe::{Hour1, Min1, Min15, Min30, Min5};
use crate::logger::init_logger;
use crate::models::bot::Bot;
use crate::models::models::{Container, SharedVec};
use crate::position_manager::PositionManager;
use crate::repository::Repository;
use log::info;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::env::home_dir;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    init_logger();

    let (bots, c) = init_dependencies();

    let app = get_router(bots, c);

    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    info!(
        "listening on port: {}",
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app).await.unwrap();
}

fn init_dependencies() -> (Arc<SharedVec<Bot>>, Arc<Container>) {
    let r = get_repository().expect("Error creating repository");
    let c = Arc::new(Container { repository: r });

    let mut bots_from_db = c
        .repository
        .get_bot_state()
        .expect("error getting bot state");

    let mut bots_name_map = HashMap::new();
    for b in bots_from_db.iter() {
        bots_name_map.insert(b.name.clone(), {});
    }

    let new_bots = init_bots();

    for b in new_bots.into_iter() {
        if !bots_name_map.contains_key(&b.name) {
            bots_from_db.push(b);
        }
    }

    let bots = Arc::new(SharedVec(UnsafeCell::new(bots_from_db)));

    let connector = BinanceConnector::new();
    let mut position_manager =
        PositionManager::new(bots.clone(), Arc::new(connector.clone()), c.clone());
    let mut entry_manager = EntryManager::new(bots.clone(), Arc::new(connector), Arc::clone(&c));

    tokio::spawn(async move {
        position_manager.start().await;
    });

    tokio::spawn(async move {
        entry_manager.start().await;
    });

    (bots, c)
}

fn init_bots() -> Vec<Bot> {
    let mut bots = Vec::new();
    let capital = 100.0;
    let leverage = 10.0;
    let take_profit_ratio = 0.8;
    let stop_loss_ratio = 0.4;
    let trailing_stop_activation_point = 0.1;

    let tf = [Min1, Min5, Min15, Min30, Hour1];
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
                bots.push(bot);
            }
        }
    }

    bots
}

fn get_repository() -> rusqlite::Result<Repository> {
    let mut path = PathBuf::from(home_dir().unwrap());
    path.push("db");

    std::fs::create_dir_all(&path).unwrap();

    path.push("traders_db.sqlite");

    Repository::new(path)
}
