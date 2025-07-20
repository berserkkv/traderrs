use crate::binance_connector::BinanceConnector;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{Candle, ManagerChannel};
use crate::strategy::strategy;

use crate::tools::wait_until_next_aligned_tick;
use chrono::{DateTime, Local, Timelike};
use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct EntryManager {
    bots: Vec<Bot>,

    bots_data: HashMap<Timeframe, HashMap<Symbol, ()>>,
    connector: BinanceConnector,
    smallest_timeframe: u64,
    channel: Arc<Mutex<ManagerChannel>>,
    from_position_manager: Receiver<Vec<Bot>>,
    for_position_manager: Sender<Vec<Bot>>,
    for_main: Sender<Vec<Bot>>,
    from_main: Receiver<Vec<Bot>>,
}

impl EntryManager {
    pub fn new(
        bots: Vec<Bot>,
        connector: BinanceConnector,
        channel: Arc<Mutex<ManagerChannel>>,
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
        debug!("Starting Entry Manager, with total {} bots", self.bots.len());
        let mut now: DateTime<Local>;
        let mut minute: usize;
        let mut hour: usize;

        let mut candles_map: HashMap<String, Vec<Candle>> = HashMap::with_capacity(self.bots.len());

        for bot in self.bots.iter() {
            info!("Bot: {:?}", bot.name);
        }

        self.send_to_main();
        wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;

        loop {
            if let Ok(mut bots) = self.from_position_manager.try_recv() {
                self.bots.append(&mut bots);
            }

            now = Local::now();
            minute = now.minute() as usize;
            hour = now.hour() as usize;


            for tf in [Timeframe::Min1, Timeframe::Min5, Timeframe::Min15] {
                if tf == Timeframe::Min1
                    || (tf == Timeframe::Min5 && minute % 5 == 0)
                    || (tf == Timeframe::Min15 && minute % 15 == 0)
                {
                    self.get_candles(tf, &mut candles_map);
                }
            }

            let mut opened_bots = Vec::with_capacity(self.bots.len());

            for bot in self.bots.iter_mut() {
                if bot.is_not_active || bot.in_pos || bot.capital < 85.0 {
                    continue;
                }

                let c = match candles_map.get(&bot.group) {
                    Some(data) if !data.is_empty() => data,
                    _ => {
                        bot.log = "Problem with connector".to_string();
                        debug!("Problem with connector, bot: {}", bot.name);
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
                if !self.for_position_manager.send(opened_bots).is_ok() {
                    error!("Failed to send opened bots");
                } else {
                    self.bots.retain(|b| !b.in_pos);
                }
            }


            candles_map.clear();

            self.send_to_main();

            debug!("Waiting for next tick...");
            wait_until_next_aligned_tick(Duration::from_secs(self.smallest_timeframe)).await;
        }
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

    fn send_to_main(&mut self) {
        if let b = &mut self.channel.try_lock().unwrap() {
            b.from_entry_manager.clear();
            b.from_entry_manager.append(&mut self.bots.clone());
        }
    }
}
