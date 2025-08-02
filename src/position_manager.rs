use crate::binance_connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::Order;
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::{DateTime, FixedOffset, Utc};
use log::{debug, error, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct PositionManager {
    bots: Arc<Vec<RwLock<Bot>>>,
    orders: Arc<RwLock<HashMap<i64, Vec<Order>>>>,
    connector: BinanceConnector,
}

impl PositionManager {
    pub fn new(
        bots: Arc<Vec<RwLock<Bot>>>,
        connector: BinanceConnector,
        orders: Arc<RwLock<HashMap<i64, Vec<Order>>>>,
    ) -> Self {
        Self {
            bots,
            connector,
            orders,
        }
    }

    pub async fn start(&mut self) {
        debug!("Starting Position Manager...");
        let sleeping_time = 2000;
        let mut prices: HashMap<Symbol, f64> = HashMap::with_capacity(self.bots.len());
        let mut to_close: Vec<Order> = Vec::with_capacity(prices.len());
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
        let mut now: DateTime<FixedOffset>;
        let mut should_close: bool;
        let mut cur_price: f64;
        loop {
            now = Utc::now().with_timezone(&offset);

            for bot in self.bots.iter() {
                let bot_read = bot.read().await;
                if !bot_read.in_pos {
                    continue;
                }
                debug!("scanning position {}", bot_read.name);

                if !prices.contains_key(&bot_read.symbol) {
                    match self.connector.get_price(&bot_read.symbol).await {
                        Ok(price) => {
                            prices.insert(bot_read.symbol, price);
                        }
                        Err(e) => {
                            error!("Error fetching price, {}", e);
                            continue;
                        }
                    }
                }
            }

            if prices.is_empty() {
                tokio::time::sleep(std::time::Duration::from_millis(sleeping_time)).await;
                continue;
            }

            for bot in self.bots.iter() {
                {
                    let bot_read = bot.read().await;
                    if !bot_read.in_pos {
                        continue;
                    }
                    if let Some(&price) = prices.get(&bot_read.symbol) {
                        should_close = should_close_position(price, &bot_read);
                        cur_price = price;
                    } else {
                        warn!("Price missing");
                        continue;
                    }
                }

                if should_close {
                    if let Ok(order) = bot.write().await.close_position(cur_price) {
                        to_close.push(order);
                    }
                } else {
                    let mut b = bot.write().await;
                    update_pnl_and_roe(&mut b, cur_price);
                    shift_stop_loss(&mut b);
                    b.last_scanned = now;
                }
            }

            self.handle_closed_position(&mut to_close).await;
            prices.clear();
            to_close.clear();
            tokio::time::sleep(std::time::Duration::from_millis(sleeping_time)).await;
        }
    }

    async fn handle_closed_position(&mut self, orders: &mut Vec<Order>) {
        if orders.is_empty() {
            return;
        }
        let mut orders_map = self.orders.write().await;

        for o in orders.iter_mut() {
            orders_map
                .entry(o.bot_id)
                .or_insert(Vec::new())
                .push(o.clone());
        }
    }
}
