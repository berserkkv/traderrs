use crate::connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::{Container, Order, SharedVec};
use crate::tools;
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::{DateTime, FixedOffset};
use log::{debug, error, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;


pub struct PositionManager {
    bots: Arc<SharedVec<Bot>>,
    connector: Arc<BinanceConnector>,
    container: Arc<Container>,
}

impl PositionManager {
    pub fn new(
        bots: Arc<SharedVec<Bot>>,
        connector: Arc<BinanceConnector>,
        container: Arc<Container>,
    ) -> Self {
        Self {
            bots,
            connector,
            container,
        }
    }

    pub async fn start(&mut self) {
        debug!("Starting Position Manager...");
        let sleep_time = 1500;
        let mut prices: HashMap<Symbol, f64> = HashMap::with_capacity(20);
        let mut to_close: Vec<Order> = Vec::with_capacity(prices.len());
        let mut now: DateTime<FixedOffset>;

        let mut fetch_tasks: Vec<JoinHandle<Option<(Symbol, f64)>>> = Vec::new();
        let mut fetch_symbols: HashMap<Symbol, ()> = HashMap::new();
        loop {
            now = tools::get_date(3);

            self.update_prices(&mut prices, &mut fetch_tasks, &mut fetch_symbols).await;

            self.scan_bots(&prices, &mut to_close, now).await;

            self.handle_closed_position(&mut to_close).await;

            tokio::time::sleep(std::time::Duration::from_millis(sleep_time)).await;
        }
    }

    async fn scan_bots(&mut self, prices: &HashMap<Symbol, f64>, to_close: &mut Vec<Order>, now: DateTime<FixedOffset>) {
        if prices.is_empty() {
            return;
        }
        unsafe {
            let bots = &mut *self.bots.0.get();

            for bot in bots.iter_mut() {
                if !bot.in_pos {
                    continue;
                }


                if let Some(&price) = prices.get(&bot.symbol) {
                    if should_close_position(price, &bot) {
                        if let Ok(order) = bot.close_position(price) {
                            to_close.push(order);
                        }
                    } else {
                        update_pnl_and_roe(bot, price);
                        shift_stop_loss(bot);
                        bot.last_scanned = now;
                    }
                } else {
                    bot.log = "price is missing".to_string();
                    warn!("Price missing");
                    continue;
                }
            }
        }
    }

    async fn handle_closed_position(&mut self, orders: &mut Vec<Order>) {
        if orders.is_empty() {
            return;
        }

        self.container.repository.create_orders(&orders).unwrap();
    }

    async fn update_prices(&self, prices: &mut HashMap<Symbol, f64>, fetch_tasks: &mut Vec<JoinHandle<Option<(Symbol, f64)>>>, fetch_symbols: &mut HashMap<Symbol, ()>) {
        fetch_tasks.clear();
        fetch_symbols.clear();
        prices.clear();

        unsafe {
            let bots = &mut *self.bots.0.get();
            for bot in bots.iter() {
                if !bot.in_pos {
                    continue;
                }
                fetch_symbols.insert(bot.symbol, {});
            }
        }

        let semaphore = Arc::new(Semaphore::new(25)); // Limit concurrent tasks


        for (smb, _) in fetch_symbols.drain() {
            let connector = Arc::clone(&self.connector);
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

            let handler: JoinHandle<Option<(Symbol, f64)>> = tokio::spawn(async move {
                let _permit = permit;
                match connector.get_price(&smb).await {
                    Ok(price) => Some((smb, price)),
                    Err(e) => {
                        error!("Error fetching price for {:?}: {}", smb, e);
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
