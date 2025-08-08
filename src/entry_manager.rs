use crate::binance_connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{Candle, Container};
use crate::strategy::strategy;
use crate::tools::{is_timeframe_now, wait_until_next_aligned_tick};
use chrono::{DateTime, FixedOffset, Local, Timelike};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct EntryManager {
    bots: Arc<Vec<RwLock<Bot>>>,
    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: Arc<BinanceConnector>,
    c: Arc<Container>,
    smallest_timeframe: u64,
}

impl EntryManager {
    pub fn new(bots: Arc<Vec<RwLock<Bot>>>, connector: Arc<BinanceConnector>, c: Arc<Container>) -> Self {
        Self {
            bots,
            bots_data: HashMap::new(),
            connector,
            c,
            smallest_timeframe: 60,
        }
    }

    pub async fn start(&mut self) {
        let mut now: DateTime<FixedOffset>;
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc

        let mut command: OrderCommand;
        let mut strategy_info: String;
        let mut candles_option: Option<&Vec<Candle>>;
        let mut strategy_name: String;
        let mut candles: &Vec<Candle>;

        let mut candles_map: HashMap<String, Vec<Candle>>;

        for bot in self.bots.iter() {
            info!("Bot: {:?}", bot.read().await.name);

            self.bots_data
                .entry(bot.read().await.timeframe)
                .or_insert(HashMap::new())
                .insert(bot.read().await.symbol, ());
        }

        wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
        tokio::time::sleep(Duration::from_secs(3)).await;

        loop {
            //init
            candles_map = HashMap::with_capacity(self.bots.len());
            now = Local::now().with_timezone(&offset);


            //resetting at midnight everyday
            if now.hour() == 0 && now.minute() == 0 {
                for b in self.bots.iter() {
                    let _ = self.c.repository.create_bot(b.read().await).expect("");
                    b.write().await.reset();
                }
            }


            self.update_candles(now.minute(), &mut candles_map).await;

            for bot_lock in self.bots.iter() {
                //read section
                {
                    let bot_read = bot_lock.read().await;
                    if bot_read.is_not_active
                      || bot_read.capital < 85.0
                      || !is_timeframe_now(&*bot_read, now.minute())
                      || bot_read.in_pos
                    {
                        continue;
                    }
                    candles_option = candles_map.get(&bot_read.group);
                    strategy_name = bot_read.strategy_name.clone();
                }

                bot_lock.write().await.last_scanned = now;

                candles = match candles_option {
                    Some(data) if !data.is_empty() => data,
                    _ => {
                        debug!("Problem with connector");
                        bot_lock.write().await.log = "Problem with connector".to_string();

                        continue;
                    }
                };

                (command, strategy_info) = strategy::get_strategy(&strategy_name).run(candles);
                debug!("command: {:?}, info: {}", command, strategy_info);

                match command {
                    OrderCommand::Long | OrderCommand::Short => {
                        if let Err(e) = bot_lock.write().await.open_position(&command, &self.connector).await {
                            error!("Failed to open position for {}: {}", bot_lock.read().await.name, e);
                        }
                    }
                    _ => {
                        bot_lock.write().await.log = strategy_info.clone();
                    }
                }
            }

            candles_map.clear();

            debug!("Waiting for next tick...");
            wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }


    async fn update_candles(&self, minute: u32, candles_map: &mut HashMap<String, Vec<Candle>>) {
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
                candles_map.insert(key, candles);
            }
        }
    }

}