use crate::enums::{OrderCommand, Symbol};
use crate::models::bot::Bot;
use chrono::{DateTime, FixedOffset, Utc};
use std::cmp::Ordering;
use std::sync::RwLock;

pub trait Strategy {
    fn name(&self) -> &str;
}

pub trait Connector {
    fn get_price(&self, symbol: &Symbol) -> f64;
}

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

#[derive(Debug)]
pub struct ManagerChannel {
    pub from_entry_manager: RwLock<Vec<Bot>>,
    pub from_position_manager: RwLock<Vec<Bot>>,
    pub orders: RwLock<Vec<Order>>,
}

impl ManagerChannel {
    pub fn new() -> Self {
        Self {
            from_entry_manager: RwLock::new(Vec::new()),
            from_position_manager: RwLock::new(Vec::new()),
            orders: RwLock::new(Vec::new()),
        }
    }

    pub fn get_bots(&self) -> Vec<Bot> {
        let mut bots = Vec::new();
        bots.append(&mut self.from_position_manager.read().unwrap().clone());
        bots.append(&mut self.from_entry_manager.read().unwrap().clone());

        bots.sort_by(|a, b| {
            a.is_not_active
                .cmp(&b.is_not_active)
                .then(cmp_f64(
                    &(a.capital + a.order_capital),
                    &(b.capital + b.order_capital),
                ))
                .then(a.timeframe.cmp(&b.timeframe))
        });
        bots
    }
}

fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => a.partial_cmp(b).unwrap(),
    }
}
