use crate::binance_connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::{ManagerChannel, Order};
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::Utc;
use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PositionManager {
    bots: Vec<Bot>,
    connector: BinanceConnector,
    channel: Arc<ManagerChannel>,
    from_entry_manager: Receiver<Vec<Bot>>,
    for_entry_manager: Sender<Vec<Bot>>,
    for_main: Sender<Vec<Bot>>,
    from_main: Receiver<Vec<Bot>>,
}

impl PositionManager {
    pub fn new(
        connector: BinanceConnector,
        channel: Arc<ManagerChannel>,
        from_entry_manager: Receiver<Vec<Bot>>,
        for_entry_manager: Sender<Vec<Bot>>,
        for_main: Sender<Vec<Bot>>,
        from_main: Receiver<Vec<Bot>>,
    ) -> Self {
        Self {
            bots: Vec::new(),
            connector,
            channel,
            from_entry_manager,
            for_entry_manager,
            for_main,
            from_main,
        }
    }

    pub async fn start(&mut self) {
        debug!("Starting Position Manager...");
        let mut prices: HashMap<Symbol, f64> = HashMap::with_capacity(self.bots.len());
        let mut to_close: Vec<Order> = Vec::with_capacity(self.bots.len());
        loop {
            self.get_from_channel().await;
            if self.bots.is_empty() {
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                continue;
            }

            for b in self.bots.iter() {
                debug!("scanning position {}", b.name);
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

            for bot in self.bots.iter_mut() {
                if !bot.in_pos {
                    continue;
                }

                let cur_price = prices[&bot.symbol];

                if !should_close_position(cur_price, bot) {
                    if let Ok(order) = bot.close_position(cur_price) {
                        to_close.push(order);
                    }
                } else {
                    update_pnl_and_roe(bot, cur_price);
                    shift_stop_loss(bot);
                    bot.last_scanned = Utc::now();
                }
            }

            self.handle_closed_position(&mut to_close).await;
            self.send_to_main().await;
            prices.clear();
            to_close.clear();
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    async fn handle_closed_position(&mut self, orders: &mut Vec<Order>) {
        if orders.is_empty() {
            return;
        }

        let mut closed_bots = Vec::with_capacity(orders.len());

        self.bots.retain(|b| {
            if b.in_pos {
                true
            } else {
                closed_bots.push(b.clone());
                false
            }
        });

        if !closed_bots.is_empty() {
            if !self.for_entry_manager.send(closed_bots).is_ok() {
                error!("Failed to send closed bots");
            }
        }

        let mut o = self.channel.orders.write().unwrap();
        o.append(orders);
    }

    async fn get_from_channel(&mut self) {
        if let Ok(mut bots) = self.from_entry_manager.try_recv() {
            self.bots.append(&mut bots);
        }
    }

    async fn send_to_main(&mut self) {
        let v = &mut self.channel.from_position_manager.write().unwrap();
        v.clear();
        v.append(&mut self.bots.clone());
    }
}
