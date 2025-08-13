use crate::binance_connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{Candle, Container, StrategyContainer};
use crate::tools::wait_until_next_aligned_tick;
use chrono::{DateTime, FixedOffset, Local, Timelike};
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinHandle;

#[derive()]
pub struct EntryManager {
    bots: Arc<Vec<RwLock<Bot>>>,
    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: Arc<BinanceConnector>,
    c: Arc<Container>,
    strategy_container: StrategyContainer,
}

impl EntryManager {
    pub fn new(bots: Arc<Vec<RwLock<Bot>>>, connector: Arc<BinanceConnector>, c: Arc<Container>) -> Self {
        Self {
            bots,
            bots_data: HashMap::new(),
            connector,
            c,
            strategy_container: StrategyContainer::new(),
        }
    }

    pub async fn start(&mut self) {
        debug!("Starting Entry Manager...");
        let mut now: DateTime<FixedOffset>;
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
        let sleep_time = 60;
        let extra_sleep_time = 3;


        let mut strategy_container: StrategyContainer = StrategyContainer::new();

        self.update_bots_data().await;

        wait_until_next_aligned_tick(Duration::from_secs(sleep_time)).await;
        tokio::time::sleep(Duration::from_secs(extra_sleep_time)).await;

        loop {
            now = Local::now().with_timezone(&offset);

            strategy_container.reset();

            self.save_and_reset_bots(&now).await;

            self.update_candles(now.minute(), &mut strategy_container).await;

            self.scan_bots(&now).await;

            wait_until_next_aligned_tick(Duration::from_secs(sleep_time)).await;
            tokio::time::sleep(Duration::from_secs(extra_sleep_time)).await;
        }
    }

    async fn update_bots_data(&mut self) {
        for bot in self.bots.iter() {
            self.bots_data
                .entry(bot.read().await.timeframe)
                .or_insert(HashMap::new())
                .insert(bot.read().await.symbol, ());
        }
    }

    async fn save_and_reset_bots(&mut self, now: &DateTime<FixedOffset>) {
        if now.hour() == 0 && now.minute() == 0 {
            for b in self.bots.iter() {
                let _ = self.c.repository.create_bot(b.read().await).expect("");
                b.write().await.reset();
            }
        }
    }

    async fn scan_bots(&mut self, now: &DateTime<FixedOffset>) {
        for bot_lock in self.bots.iter() {
            if bot_lock.read().await.is_allowed_for_scanning(now) {
                continue;
            }

            bot_lock.write().await.last_scanned = *now;

            let (command, strategy_info) = bot_lock.read().await.run_strategy(&self.strategy_container);

            debug!("command: {:?}, info: {}", command, strategy_info);

            match command {
                OrderCommand::Long | OrderCommand::Short => {
                    if let Err(e) = bot_lock.write().await.open_position(&command, &self.connector).await {
                        error!("Failed to open position for {}: {}", bot_lock.read().await.name, e);
                    }
                }
                _ => {
                    bot_lock.write().await.log = strategy_info;
                }
            }
        }
    }

    async fn update_candles(&self, minute: u32, strategy_container: &mut StrategyContainer) {
        let connector = Arc::clone(&self.connector);
        let semaphore = Arc::new(Semaphore::new(20)); // Limit concurrent tasks
        let mut fetch_tasks = Vec::new();

        let timeframes_to_fetch = [
            Timeframe::Min1,
            Timeframe::Min5,
            Timeframe::Min15
        ].into_iter().filter(|tf| {
            *tf == Timeframe::Min1
              || (*tf == Timeframe::Min5 && minute % 5 == 0)
              || (*tf == Timeframe::Min15 && minute % 15 == 0)
        });

        for tf in timeframes_to_fetch {
            if let Some(inner_map) = self.bots_data.get(&tf) {
                for smb in inner_map.keys() {
                    let smb_copy = smb.clone();
                    let key = format!("{:?}{:?}", tf, smb);
                    let connector = Arc::clone(&connector);
                    let tf_copy = tf;
                    let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

                    let handle: JoinHandle<Option<(String, Vec<Candle>)>> = tokio::spawn(async move {
                        let _permit = permit; // Drop when task is done
                        match connector.get_candles(smb_copy, tf_copy, 202).await {
                            Ok(candles) => Some((key, candles)),
                            Err(e) => {
                                error!("Error fetching candles: {}", e);
                                None
                            }
                        }
                    });

                    fetch_tasks.push(handle);
                }
            }
        }

        debug!("min: {}, tasks: {}", minute, fetch_tasks.len());

        for task in fetch_tasks {
            if let Ok(Some((key, candles))) = task.await {
                strategy_container.candles_map.insert(key, candles);
            }
        }
    }
}