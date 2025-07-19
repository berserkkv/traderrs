use crate::binance_connector::BinanceConnector;
use crate::calculator::{calculate_pnl, calculate_roe};
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::{Bot, Bots};
use crate::models::models::{Candle, ManagerChannel};
use crate::strategy::strategy;

use crate::tools::wait_until_next_aligned_tick;
use chrono::{Local, Timelike, Utc};
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct BotManager {
    pub bots: Vec<Bot>,
    pub bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    pub connector: BinanceConnector,
    pub smallest_timeframe: u64,
    pub channel: Arc<Mutex<ManagerChannel>>,
}

impl BotManager {
    pub fn new(bots: Vec<Bot>, connector: BinanceConnector, channel: Arc<Mutex<ManagerChannel>>) -> Self {
        Self {
            bots,
            bots_data: HashMap::new(),
            connector,
            smallest_timeframe: 60,
            channel,
        }
    }

    pub async fn start(&mut self) {
        loop {
            debug!("Waiting for next tick...");
            wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;

            let now = Local::now();
            let minute = now.minute() as usize;
            let hour = now.hour() as usize;

            self.scan_bots(minute, hour).await;
        }
    }

    pub async fn scan_bots(&mut self, minute: usize, _hour: usize) {
        debug!("Scanning bots");

        let mut candles: HashMap<String, Vec<Candle>> = HashMap::new();

        for tf in [Timeframe::Min1, Timeframe::Min5, Timeframe::Min15] {
            if tf == Timeframe::Min1 || (tf == Timeframe::Min5 && minute % 5 == 0) || (tf == Timeframe::Min15 && minute % 15 == 0) {
                self.get_candles(tf, &mut candles);
            }
        }

        info!("Scanned {} candles", candles.len());

        let mut opened_bots = Vec::new();

        for bot in self.bots.iter_mut() {
            if bot.is_not_active || bot.in_pos || bot.capital < 85.0 { continue; }

            info!("Scanning bot {}", bot.name);

            let key = format!("{:?}{:?}", bot.timeframe, bot.symbol);


            let c = match candles.get(&key) {
                Some(data) if !data.is_empty() => data,
                _ => {
                    bot.log = "Problem with connector".to_string();
                    continue;
                }
            };

            let cmd = strategy::get_strategy(&bot.strategy_name).run(c);
            info!("bot: {}, command {:?}", bot.name, cmd.0);


            match cmd.0 {
                OrderCommand::Long | OrderCommand::Short => {
                    if bot.open_position(&cmd.0).is_ok() {
                        opened_bots.push(bot.clone());
                    }
                }
                _ => {}
            }
        }

        if !opened_bots.is_empty() {
            self.send_to_channel(&mut opened_bots).await;
        }
    }

    async fn send_to_channel(&mut self, bots: &mut Vec<Bot>) {
        let mut c = self.channel.lock().await;
        c.for_position_manager.append(bots);
        self.bots.retain(|b| !b.in_pos);
    }

    fn get_candles(&self, tf: Timeframe, candles: &mut HashMap<String, Vec<Candle>>) {
        if let Some(set) = self.bots_data.get(&tf) {
            for smb in set.keys() {
                let key = format!("{:?}{:?}", tf, smb);
                if !candles.contains_key(&key) {
                    let data = self.connector.get_candles(*smb, tf, 202);
                    candles.insert(key, data);
                }
            }
        }
    }
}
