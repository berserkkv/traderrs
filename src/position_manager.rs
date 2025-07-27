use crate::binance_connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::Order;
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::{DateTime, FixedOffset, Utc};
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct PositionManager {
    bots: Arc<RwLock<Vec<Bot>>>,
    connector: BinanceConnector,
}

impl PositionManager {
    pub fn new(bots: Arc<RwLock<Vec<Bot>>>, connector: BinanceConnector) -> Self {
        Self { bots, connector }
    }

    pub async fn start(&mut self) {
        debug!("Starting Position Manager...");
        let sleeping_time = 2000;
        let mut prices: HashMap<Symbol, f64> = HashMap::with_capacity(self.bots.read().await.len());
        let mut to_close: Vec<Order> = Vec::with_capacity(prices.len());
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
        let mut now: DateTime<FixedOffset>;
        loop {
            now = Utc::now().with_timezone(&offset);

            for b in self.bots.read().await.iter() {
                debug!("scanning position {}", b.name);
                if !b.in_pos {
                    continue;
                }
                if !prices.contains_key(&b.symbol) {
                    match self.connector.get_price(&b.symbol).await {
                        Ok(price) => {
                            prices.insert(b.symbol, price);
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

            for bot in self.bots.write().await.iter_mut() {
                if !bot.in_pos {
                    continue;
                }

                let cur_price = prices[&bot.symbol];

                if should_close_position(cur_price, bot) {
                    if let Ok(order) = bot.close_position(cur_price) {
                        to_close.push(order);
                    }
                } else {
                    update_pnl_and_roe(bot, cur_price);
                    shift_stop_loss(bot);
                    bot.last_scanned = now;
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

        // todo push orders to vec
    }
}
