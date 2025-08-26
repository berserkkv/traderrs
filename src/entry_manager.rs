use crate::connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{Candle, Container, SharedVec, StrategyContainer};
use crate::tools;
use crate::tools::wait_until_next_aligned_tick;
use chrono::{DateTime, FixedOffset, Timelike};
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

#[derive()]
pub struct EntryManager {
    bots: Arc<SharedVec<Bot>>,
    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: Arc<BinanceConnector>,
    c: Arc<Container>,
    strategy_container: StrategyContainer,
}

impl EntryManager {
    pub fn new(bots: Arc<SharedVec<Bot>>, connector: Arc<BinanceConnector>, c: Arc<Container>) -> Self {
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
        let sleep_time = 60;
        let extra_sleep_time = 3;
        let bots: &mut Vec<Bot>;
        unsafe {
            bots = &mut *self.bots.0.get();
        }


        self.update_bots_data(bots).await;

        wait_until_next_aligned_tick(Duration::from_secs(sleep_time)).await;
        tokio::time::sleep(Duration::from_secs(extra_sleep_time)).await;

        loop {
            now = tools::get_date(3);

            self.save_and_reset_bots(bots, &now).await;

            self.update_candles(now.minute()).await;

            self.calculate_ta().await;

            self.scan_bots(bots, &now).await;

            self.strategy_container.reset();

            wait_until_next_aligned_tick(Duration::from_secs(sleep_time)).await;
            tokio::time::sleep(Duration::from_secs(extra_sleep_time)).await;
        }
    }

    async fn scan_bots(&mut self, bots: &mut Vec<Bot>, now: &DateTime<FixedOffset>) {
        for bot in bots.iter_mut() {
            if bot.is_not_allowed_for_scanning(now) { continue; }

            bot.last_scanned = *now;

            let (command, strategy_info) = bot.run_strategy(&self.strategy_container);

            debug!("command: {:?}, info: {}", command, strategy_info);

            match command {
                OrderCommand::Long | OrderCommand::Short => {
                    if let Err(e) = bot.open_position(&command, &self.connector).await {
                        error!("Failed to open position for {}: {}", bot.name, e);
                    }
                }
                _ => {
                    bot.log = strategy_info;
                }
            }
        }
    }

    async fn update_bots_data(&mut self, bots: &mut Vec<Bot>) {
        self.bots_data.clear();

        for bot in bots.iter() {
            self.bots_data
                .entry(bot.timeframe)
                .or_insert(HashMap::new())
                .insert(bot.symbol, ());
        }
    }

    async fn save_and_reset_bots(&mut self, bots: &mut Vec<Bot>, now: &DateTime<FixedOffset>) {
        if now.hour() == 0 && now.minute() == 0 {
            self.c.repository.create_bots_in_batch(bots).expect("error creating bots");

            for b in bots.iter_mut() {
                b.reset();
            }
        }
    }

    async fn update_candles(&mut self, minute: u32) {
        let connector = Arc::clone(&self.connector);
        let semaphore = Arc::new(Semaphore::new(25)); // Limit concurrent tasks
        let mut fetch_tasks = Vec::new();

        let timeframes_to_fetch = [
            Timeframe::Min1,
            Timeframe::Min5,
            Timeframe::Min15,
            Timeframe::Min30,
            Timeframe::Hour1,
        ].into_iter().filter(|tf| {
            *tf == Timeframe::Min1
              || (*tf == Timeframe::Min5 && minute % 5 == 0)
              || (*tf == Timeframe::Min15 && minute % 15 == 0)
              || (*tf == Timeframe::Min30 && minute % 30 == 0)
              || (*tf == Timeframe::Hour1 && minute % 60 == 0)
        });

        for tf in timeframes_to_fetch {
            if let Some(inner_map) = self.bots_data.get(&tf) {
                for smb in inner_map.keys() {
                    let smb_copy = smb.clone();
                    let key = (tf, *smb);
                    let connector = Arc::clone(&connector);
                    let tf_copy = tf;
                    let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

                    let handle: JoinHandle<Option<((Timeframe, Symbol), Vec<Candle>)>> = tokio::spawn(async move {
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

        debug!("minute: {}, tasks: {}", minute, fetch_tasks.len());

        for task in fetch_tasks {
            if let Ok(Some((key, candles))) = task.await {
                self.strategy_container.candles_map.insert(key, candles);
            }
        }
    }

    async fn calculate_ta(&mut self) {
        self.strategy_container.calculate_all();
    }
}