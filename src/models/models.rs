use crate::bot_manager::BotManager;
use crate::enums::{OrderCommand, Symbol};
use crate::models::bot::Bot;
use crate::position_manager::PositionManager;
use chrono::{DateTime, Utc};

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
    pub time: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
    pub fee: f64,
    pub leverage: f64,
}

#[derive(Debug)]
pub struct ManagerChannel {
    pub for_bot_manager: Vec<Bot>,
    pub for_position_manager: Vec<Bot>,
}

impl ManagerChannel {
    pub fn new(for_bot_manager: Vec<Bot>, for_position_manager: Vec<Bot>) -> Self {
        Self { for_bot_manager, for_position_manager }
    }

    pub fn get_bots(&self) -> Vec<Bot> {
        let mut bots = Vec::new();
        bots.append(&mut self.for_bot_manager.clone());
        bots.append(&mut self.for_position_manager.clone());
        bots
    }
}

#[derive(Debug)]
pub struct Container<'a> {
    pub bot_manager: &'a BotManager,
    pub position_manager: &'a PositionManager,
}

impl<'a> Container<'a> {
    pub fn new(bot_manager: &'a BotManager, position_manager: &'a PositionManager) -> Self {
        Self { bot_manager, position_manager }
    }

    pub fn get_bots(&self) -> Vec<Bot> {
        let mut bots = Vec::new();
        bots.append(self.bot_manager.bots.clone().as_mut());

        bots
    }
}