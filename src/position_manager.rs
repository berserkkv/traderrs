use crate::binance_connector::BinanceConnector;
use crate::enums::Symbol;
use crate::models::bot::Bot;
use crate::models::models::{ManagerChannel, Order};
use crate::tools::{shift_stop_loss, should_close_position, update_pnl_and_roe};
use chrono::Utc;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PositionManager {
    pub bots: Vec<Bot>,
    pub connector: BinanceConnector,
    pub channel: Arc<Mutex<ManagerChannel>>,
}

impl PositionManager {
    pub fn new(connector: BinanceConnector, channel: Arc<Mutex<ManagerChannel>>) -> Self {
        Self { bots: Vec::new(), connector, channel }
    }

    pub async fn monitor(&mut self) {
        if self.bots.is_empty() {
            debug!("No bots in open position");
            return;
        }

        let mut prices: HashMap<Symbol, f64> = HashMap::new();


        for b in self.bots.iter() {
            if !prices.contains_key(&b.symbol) {
                let price = self.connector.get_price(&b.symbol);
                prices.insert(b.symbol, price);
            }
        }

        let mut to_close = Vec::new();

        for bot in self.bots.iter_mut() {
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
                bot.order_scanned_at = Utc::now();
            }
        }

        self.handle_closed_position(to_close).await;
        self.get_from_channel().await;
    }

    async fn handle_closed_position(&mut self, orders: Vec<Order>) {
        if orders.is_empty() { return; }

        for order in orders.iter() {
            info!("Closed order for bot {}: {:?}", order.bot_id, order);
        }
        let mut closed_bots = Vec::new();
        self.bots.retain(|b| {
            if b.in_pos {
                true
            } else {
                closed_bots.push(b.clone());
                false
            }
        });

        if !closed_bots.is_empty() {
            let mut channel = self.channel.lock().await;
            channel.for_bot_manager.append(&mut closed_bots);
        }
    }

    async fn get_from_channel(&mut self) {
        let mut channel = self.channel.lock().await;
        self.bots.append(&mut channel.for_position_manager);
    }
}