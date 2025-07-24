use crate::binance_connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{Candle, ManagerChannel};
use crate::strategy::strategy;

use crate::tools::{is_timeframe_now, wait_until_next_aligned_tick};
use chrono::{DateTime, FixedOffset, Local, Timelike};
use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct EntryManager {
    bots: Vec<Bot>,

    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: BinanceConnector,
    smallest_timeframe: u64,
    channel: Arc<ManagerChannel>,
    from_position_manager: Receiver<Vec<Bot>>,
    for_position_manager: Sender<Vec<Bot>>,
    for_main: Sender<Vec<Bot>>,
    from_main: Receiver<Vec<Bot>>,
}

impl EntryManager {
    pub fn new(
        bots: Vec<Bot>,
        connector: BinanceConnector,
        channel: Arc<ManagerChannel>,
        from_position_manager: Receiver<Vec<Bot>>,
        for_position_manager: Sender<Vec<Bot>>,
        for_main: Sender<Vec<Bot>>,
        from_main: Receiver<Vec<Bot>>,
    ) -> Self {
        Self {
            bots,
            bots_data: HashMap::new(),
            connector,
            smallest_timeframe: 60,
            channel,
            from_position_manager,
            for_position_manager,
            for_main,
            from_main,
        }
    }

    pub async fn start(&mut self) {
        debug!(
            "Starting Entry Manager, with total {} bots",
            self.bots.len()
        );
        let mut now: DateTime<FixedOffset>;
        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc

        let mut minute: usize;
        let mut command: OrderCommand;
        let mut strategy_info: String;

        let mut candles_map: HashMap<String, Vec<Candle>>;
        let mut opened_bots: Vec<Bot>;

        for bot in self.bots.iter() {
            info!("Bot: {:?}", bot.name);
        }

        for b in self.bots.iter() {
            self.bots_data
                .entry(b.timeframe)
                .or_insert(HashMap::new())
                .insert(b.symbol, ());
        }

        self.send_to_main();
        wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
        tokio::time::sleep(Duration::from_secs(3)).await;

        loop {
            if let Ok(mut bots) = self.from_position_manager.try_recv() {
                self.bots.append(&mut bots);
            }

            //init
            opened_bots = Vec::with_capacity(self.bots.len());
            candles_map = HashMap::with_capacity(self.bots.len());
            now = Local::now().with_timezone(&offset);
            minute = now.minute() as usize;

            self.update_candles(minute, &mut candles_map).await;

            for bot in self.bots.iter_mut() {
                if bot.is_not_active || bot.capital < 85.0 || !is_timeframe_now(&bot, minute) {
                    continue;
                }

                if bot.in_pos {
                    continue;
                }

                bot.last_scanned = now;

                let c = match candles_map.get(&bot.group) {
                    Some(data) if !data.is_empty() => data,
                    _ => {
                        bot.log = "Problem with connector".to_string();
                        debug!("Problem with connector, bot: {}", bot.name);
                        continue;
                    }
                };

                (command, strategy_info) = strategy::get_strategy(&bot.strategy_name).run(c);

                match command {
                    OrderCommand::Long | OrderCommand::Short => {
                        if bot.open_position(&command, &self.connector).await.is_ok() {
                            opened_bots.push(bot.clone());
                        }
                    }
                    _ => {
                        bot.log = strategy_info.clone();
                    }
                }
            }

            if !opened_bots.is_empty() {
                if self.for_position_manager.send(opened_bots).is_ok() {
                    self.bots.retain(|b| !b.in_pos);
                } else {
                    error!("Failed to send opened bots through channel");
                }
            }

            candles_map.clear();

            self.send_to_main();

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

    fn send_to_main(&mut self) {
        let v = &mut self.channel.from_entry_manager.write().unwrap();
        v.clear();
        v.append(&mut self.bots.clone());
    }
}
