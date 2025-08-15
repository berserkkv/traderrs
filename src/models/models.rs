use crate::enums::{OrderCommand, Symbol};
use crate::repository::Repository;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Candle {
    pub close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub open_time: u64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub symbol: Symbol,
    pub order_type: OrderCommand,
    pub bot_id: i64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub quantity: f64,
    pub pnl: f64,
    pub roe: f64,
    pub created_at: DateTime<FixedOffset>,
    pub closed_at: DateTime<FixedOffset>,
    pub fee: f64,
    pub leverage: f64,
}
impl Order {
    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    pub fn dummy() -> Self {
        Self {
            symbol: Symbol::SolUsdt,
            order_type: OrderCommand::Long,
            bot_id: 1,
            entry_price: 100.0,
            exit_price: 101.0,
            quantity: 1.0,
            pnl: 0.0,
            roe: 0.0,
            created_at: DateTime::default(),
            closed_at: DateTime::default(),
            fee: 0.1,
            leverage: 10.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub(crate) cpu_usage: f32,
    pub(crate) memory_usage: u64,
    pub started_time: DateTime<FixedOffset>,
}



#[derive(Debug, Clone)]
pub struct Container {
    pub repository: Repository,
}



#[derive(Debug)]
pub struct StrategyContainer {
    pub candles_map: HashMap<String, Vec<Candle>>,
    pub vec_map: HashMap<String, Vec<f64>>,
    pub last_map: HashMap<String, f64>,
}
impl StrategyContainer {
    pub fn new() -> Self {
        Self {
            candles_map: HashMap::new(),
            vec_map: HashMap::new(),
            last_map: HashMap::new(),
        }
    }
    pub fn reset(&mut self) {
        self.candles_map.clear();
        self.vec_map.clear();
        self.last_map.clear()
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatisticResult {
    pub name: String,
    pub capital: f64,
    pub wins: u16,
    pub losses: u16,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotStatistic{
    pub bot_name: String,
    pub win_days: u16,
    pub lose_days: u16,
    pub capital: f64,
    pub results: Vec<StatisticResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Statistic {
    pub bot_statistics: Vec<BotStatistic>,
}


