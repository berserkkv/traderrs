use crate::enums::{OrderCommand, Symbol};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Candle {
    pub close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub open_time: u64,
    pub volume: f64,
}

#[derive(Debug)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub(crate) cpu_usage: f32,
    pub(crate) memory_usage: u64,
}
