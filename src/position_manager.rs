use crate::binance_connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::Order;
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::{DateTime, FixedOffset, Utc};
use log::{debug, error, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinHandle;

#[derive()]
pub struct PositionManager {
    bots: Arc<Vec<RwLock<Bot>>>,
    orders: Arc<RwLock<HashMap<i64, Vec<Order>>>>,
    connector: Arc<BinanceConnector>,
}

impl PositionManager {
    pub fn new(
        bots: Arc<Vec<RwLock<Bot>>>,
        connector: Arc<BinanceConnector>,
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
        let sleep_time = 2000;
        let mut prices: HashMap<Symbol, f64> = HashMap::with_capacity(self.bots.len());
        let mut to_close: Vec<Order> = Vec::with_capacity(prices.len());
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
        let mut now: DateTime<FixedOffset>;

        let mut fetch_tasks: Vec<JoinHandle<Option<(Symbol, f64)>>> = Vec::new();
        let mut fetch_symbols: HashMap<Symbol, ()> = HashMap::new();
        loop {
            now = Utc::now().with_timezone(&offset);

            self.update_prices(&mut prices, &mut fetch_tasks, &mut fetch_symbols).await;

            self.scan_bots(&prices, &mut to_close, now).await;

            self.handle_closed_position(&mut to_close).await;
            prices.clear();
            to_close.clear();
            tokio::time::sleep(std::time::Duration::from_millis(sleep_time)).await;
        }
    }

    async fn scan_bots(&mut self, prices: &HashMap<Symbol, f64>, to_close: &mut Vec<Order>, now: DateTime<FixedOffset>) {
        if prices.is_empty() {
            return;
        }

        for bot in self.bots.iter() {
            let cur_price: f64;
            let should_close: bool;
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

    async fn update_prices(&self, prices: &mut HashMap<Symbol, f64>, fetch_tasks: &mut Vec<JoinHandle<Option<(Symbol, f64)>>>, fetch_symbols: &mut HashMap<Symbol, ()>) {
        fetch_tasks.clear();
        fetch_symbols.clear();

        for bot in self.bots.iter() {
            let bot_read = bot.read().await;
            if !bot_read.in_pos {
                continue;
            }

            fetch_symbols.insert(bot_read.symbol, {});
        }
        let semaphore = Arc::new(Semaphore::new(20)); // Limit concurrent tasks


        for smb in fetch_symbols.keys() {
            let smb_copy = smb.clone();
            let connector = Arc::clone(&self.connector);
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

            let handler: JoinHandle<Option<(Symbol, f64)>> = tokio::spawn(async move {
                let _permit = permit;
                match connector.get_price(&smb_copy).await {
                    Ok(price) => Some((smb_copy, price)),
                    Err(e) => {
                        error!("Error fetching price for {:?}: {}", smb_copy, e);
                        None
                    }
                }
            });
            fetch_tasks.push(handler);
        }

        for task in fetch_tasks {
            if let Ok(Some((smb, price))) = task.await {
                prices.insert(smb, price);
            }
        }
    }
}
