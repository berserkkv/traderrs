use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::repository::Repository;
use crate::{ta, tools};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::cell::UnsafeCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
    pub bot_name: String,
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
            bot_name: "dummy".to_string(),
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
    pub candles_map: HashMap<(Timeframe, Symbol), Vec<Candle>>,
    pub macd: HashMap<(Timeframe, Symbol), Macd>,
    pub ema: HashMap<(Timeframe, Symbol, usize), Vec<f64>>,
}
impl StrategyContainer {
    pub fn new() -> Self {
        Self {
            candles_map: HashMap::new(),
            macd: HashMap::new(),
            ema: HashMap::new(),
        }
    }
    pub fn reset(&mut self) {
        self.candles_map.clear();
        self.macd.clear();
        self.ema.clear();
    }

    pub fn calculate_all(&mut self) {
        for ((timeframe, symbol), candles) in self.candles_map.iter() {
            let (macd, signal, histogram) = ta::macd_slice(&tools::get_close_prices(&candles));
            self.macd.insert((*timeframe, *symbol), Macd { macd, signal, histogram });
            self.ema.insert((*timeframe, *symbol, 20), ta::ema_slice(&tools::get_close_prices(candles), 20));
            self.ema.insert((*timeframe, *symbol, 50), ta::ema_slice(&tools::get_close_prices(candles), 50));
            self.ema.insert((*timeframe, *symbol, 200), ta::ema_slice(&tools::get_close_prices(candles), 200));
        }
    }

    pub fn get_macd(&self, timeframe: &Timeframe, symbol: &Symbol) -> Option<&Macd> {
        self.macd.get(&(*timeframe, *symbol))
    }

    pub fn get_ema(&self, timeframe: &Timeframe, symbol: &Symbol, period: usize) -> Option<&Vec<f64>> {
        self.ema.get(&(*timeframe, *symbol, period))
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
pub struct BotStatistic {
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


#[derive(Debug, Clone)]
pub struct Macd {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
    pub histogram: Vec<f64>,
}


pub struct SharedVec<T>(pub UnsafeCell<Vec<T>>);

unsafe impl<T> Send for SharedVec<T> {}
unsafe impl<T> Sync for SharedVec<T> {}