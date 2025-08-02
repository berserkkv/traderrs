use crate::binance_connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::Candle;
use crate::strategy::strategy;

use crate::tools::{is_timeframe_now, wait_until_next_aligned_tick};
use chrono::{DateTime, FixedOffset, Local, Timelike};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct EntryManager {
    bots: Arc<RwLock<Vec<Bot>>>,
    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: BinanceConnector,
    smallest_timeframe: u64,
}

impl EntryManager {
    pub fn new(bots: Arc<RwLock<Vec<Bot>>>, connector: BinanceConnector) -> Self {
        Self {
            bots,
            bots_data: HashMap::new(),
            connector,
            smallest_timeframe: 60,
        }
    }

    pub async fn start(&mut self) {
        let mut now: DateTime<FixedOffset>;
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc

        let mut minute: usize;
        let mut command: OrderCommand;
        let mut strategy_info: String;

        let mut candles_map: HashMap<String, Vec<Candle>>;

        for bot in self.bots.read().await.iter() {
            info!("Bot: {:?}", bot.name);

            self.bots_data
                .entry(bot.timeframe)
                .or_insert(HashMap::new())
                .insert(bot.symbol, ());
        }

        wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
        tokio::time::sleep(Duration::from_secs(3)).await;

        loop {
            //init
            candles_map = HashMap::with_capacity(self.bots.read().await.len());
            now = Local::now().with_timezone(&offset);
            minute = now.minute() as usize;

            self.update_candles(minute, &mut candles_map).await;

            for bot in self.bots.write().await.iter_mut() {
                if bot.is_not_active
                    || bot.capital < 85.0
                    || !is_timeframe_now(&bot, minute)
                    || bot.in_pos
                {
                    continue;
                }

                bot.last_scanned = now;

                let c = match candles_map.get(&bot.group) {
                    Some(data) if !data.is_empty() => data,
                    _ => {
                        bot.log = "Problem with connector".to_string();
                        warn!("Problem with connector, bot: {}", bot.name);
                        continue;
                    }
                };

                (command, strategy_info) = strategy::get_strategy(&bot.strategy_name).run(c);

                match command {
                    OrderCommand::Long | OrderCommand::Short => {
                        if bot.open_position(&command, &self.connector).await.is_ok() {}
                    }
                    _ => {
                        bot.log = strategy_info.clone();
                    }
                }
            }

            candles_map.clear();

            debug!("Waiting for next tick...");
            wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }

    async fn update_candles(
        &mut self,
        minute: usize,
        candles_map: &mut HashMap<String, Vec<Candle>>,
    ) {
        for tf in [Timeframe::Min1, Timeframe::Min5, Timeframe::Min15] {
            if tf == Timeframe::Min1
                || (tf == Timeframe::Min5 && minute % 5 == 0)
                || (tf == Timeframe::Min15 && minute % 15 == 0)
            {
                self.get_candles(tf, candles_map).await;
            }
        }
    }

    async fn get_candles(&self, tf: Timeframe, candles_map: &mut HashMap<String, Vec<Candle>>) {
        if let Some(set) = self.bots_data.get(&tf) {
            for smb in set.keys() {
                let key = format!("{:?}{:?}", tf, smb);
                if !candles_map.contains_key(&key) {
                    match self.connector.get_candles(*smb, tf, 202).await {
                        Ok(candles) => candles_map.insert(key, candles),
                        Err(e) => {
                            error!("Error fetching candles, {}", e);
                            continue;
                        }
                    };
                }
            }
        }
    }
}
