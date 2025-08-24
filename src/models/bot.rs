use crate::calculator::{
    calculate_buy_quantity, calculate_maker_fee, calculate_pnl, calculate_roe, calculate_stop_loss,
    calculate_take_profit, calculate_taker_fee,
};
use crate::connector::BinanceConnector;
use crate::enums::Symbol::SolUsdt;
use crate::enums::Timeframe::Min1;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::models::{Order, StrategyContainer};
use crate::strategy::strategy;
use crate::strategy::strategy::Strategy;
use crate::tools;
use crate::tools::is_timeframe_now;
use chrono::{DateTime, FixedOffset, Timelike, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub name: String,
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    pub strategy_name: String,
    #[serde(skip)]
    pub strategy: Option<Arc<dyn Strategy + Send + Sync>>,
    pub capital: f64,
    pub group: String,
    pub is_not_active: bool,

    pub wins: i16,
    pub losses: i16,

    pub log: String,
    pub started_at: DateTime<FixedOffset>,
    pub last_scanned: DateTime<FixedOffset>,

    pub leverage: f64,
    pub take_profit_ratio: f64,
    pub stop_loss_ratio: f64,
    pub is_trailing_stop_active: bool,
    pub trailing_stop_activation_point: f64,

    pub in_pos: bool,
    pub order_type: OrderCommand,
    pub order_created_at: DateTime<FixedOffset>,
    pub order_scanned_at: DateTime<FixedOffset>,
    pub order_quantity: f64,
    pub order_capital: f64,
    pub order_capital_with_leverage: f64,
    pub order_entry_price: f64,
    pub order_stop_loss: f64,
    pub order_take_profit: f64,
    pub order_fee: f64,
    pub pnl: f64,
    pub roe: f64,
}
impl Bot {
    pub fn new(
        timeframe: Timeframe,
        symbol: Symbol,
        strategy_name: String,
        capital: f64,
        leverage: f64,
        take_profit_ratio: f64,
        stop_loss_ratio: f64,
        trailing_stop_activation_ratio: f64,
    ) -> Self {
        let name = format!(
            "{}_{}_{}",
            strategy_name,
            tools::format_timeframe(&timeframe),
            format_symbol(&symbol)
        );
        let now = tools::get_date(3);
        let strategy = strategy::get_strategy(&strategy_name);

        Self {
            name,
            symbol,
            is_not_active: false,
            timeframe,
            strategy_name,
            group: format!("{:?}{:?}", timeframe, symbol),
            capital,
            last_scanned: now,
            started_at: now,
            log: "".to_string(),
            strategy: Some(strategy),

            wins: 0,
            losses: 0,

            leverage,
            take_profit_ratio,
            stop_loss_ratio,
            is_trailing_stop_active: true,
            trailing_stop_activation_point: trailing_stop_activation_ratio,

            in_pos: false,
            order_type: OrderCommand::Wait,
            order_created_at: now,
            order_scanned_at: now,
            order_quantity: 0.0,
            order_capital: 0.0,
            order_capital_with_leverage: 0.0,
            order_entry_price: 0.0,
            order_stop_loss: 0.0,
            order_take_profit: 0.0,
            order_fee: 0.0,
            pnl: 0.0,
            roe: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.started_at = tools::get_date(3);
        self.capital = 100.0;
        self.is_not_active = false;
        self.in_pos = false;
        self.losses = 0;
        self.wins = 0;
        self.order_capital = 0.0;
        self.order_capital_with_leverage = 0.0;
        self.order_fee = 0.0;
        self.pnl = 0.0;
        self.roe = 0.0;
    }

    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    pub fn new_dummy() -> Self {
        let name = format!("{}", "Dummy");
        let now = tools::get_date(3);

        Self {
            name,
            symbol: SolUsdt,
            is_not_active: false,
            timeframe: Min1,
            strategy_name: "Macd".to_string(),
            capital: 100.0,
            last_scanned: now,
            started_at: now,
            log: "".to_string(),
            group: "".to_string(),
            strategy: None,

            wins: 0,
            losses: 0,

            leverage: 10.0,
            take_profit_ratio: 1.0,
            stop_loss_ratio: 0.5,
            is_trailing_stop_active: true,
            trailing_stop_activation_point: 1.0,

            in_pos: false,
            order_type: OrderCommand::Wait,
            order_created_at: DateTime::from(now),
            order_scanned_at: DateTime::from(now),
            order_quantity: 0.0,
            order_capital: 0.0,
            order_capital_with_leverage: 0.0,
            order_entry_price: 0.0,
            order_stop_loss: 0.0,
            order_take_profit: 0.0,
            order_fee: 0.0,
            pnl: 0.0,
            roe: 0.0,
        }
    }

    pub fn can_open_position(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.is_not_active {
            debug!(
                "bot can't open position, bot not active, name: {}",
                self.name
            );
            return Err("bot can't open position, bot not active".into());
        }
        if self.in_pos {
            debug!(
                "bot can't open position, bot is already in open position, name: {}",
                self.name
            );
            return Err("bot can't open position, bot is already in open position".into());
        }
        if self.capital <= 85.0 {
            debug!(
                "bot can't open position, capital <= 85.0, name: {}",
                self.name
            );
            return Err("bot can't open position, capital <= 85.0, name".into());
        }

        Ok(())
    }

    pub async fn open_position(&mut self, command: &OrderCommand, connector: &BinanceConnector) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.can_open_position()?;

        let price = connector.get_price(&self.symbol).await?;

        self.order_type = *command;
        self.order_stop_loss = calculate_stop_loss(price, self.stop_loss_ratio, &self.order_type);
        self.order_take_profit =
          calculate_take_profit(price, self.take_profit_ratio, &self.order_type);

        let mut capital = self.capital;
        self.capital -= capital;

        let fee = calculate_taker_fee(capital);
        capital -= fee;

        let now = tools::get_date(3);

        self.order_capital_with_leverage = self.leverage * capital;
        self.order_capital = capital;
        self.order_quantity = calculate_buy_quantity(price, self.order_capital_with_leverage);
        self.order_entry_price = price;
        self.order_created_at = now;
        self.order_scanned_at = now;
        self.order_fee = fee;

        info!("Position opened: name: {}, cpt: {:.2}, type: {:?}, entry_price: {:.2}, stop_loss: {:.2}, take_profit: {:.2}, asset: {}:.2",
            self.name,
            self.order_capital,
            self.order_type,
            self.order_entry_price,
            self.order_stop_loss,
            self.order_take_profit,
            self.order_quantity,
        );

        self.in_pos = true;
        Ok(())
    }

    pub fn close_position(&mut self, cur_price: f64) -> Result<Order, Box<dyn Error>> {
        if self.order_type == OrderCommand::Wait {
            return Err("No open position to close".into());
        }
        let fee = calculate_maker_fee(self.order_capital);
        self.order_capital_with_leverage -= fee;
        self.order_capital -= fee;

        let pnl = calculate_pnl(
            cur_price,
            self.order_capital_with_leverage,
            self.order_quantity,
            &self.order_type,
        );
        let roe = calculate_roe(
            self.order_entry_price,
            cur_price,
            self.leverage,
            &self.order_type,
        );

        let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
        let now = Utc::now().with_timezone(&offset);

        self.update_statistics(pnl);

        self.order_fee += fee;

        self.capital += self.order_capital + pnl;

        let closed_order = Order {
            symbol: self.symbol,
            order_type: self.order_type,
            bot_name: self.name.clone(),
            entry_price: self.order_entry_price,
            exit_price: cur_price,
            quantity: self.order_quantity,
            pnl,
            roe,
            created_at: self.order_created_at,
            closed_at: now,
            fee: self.order_fee,
            leverage: self.leverage,
        };

        // reset position state

        self.order_entry_price = 0.0;
        self.order_stop_loss = 0.0;
        self.order_take_profit = 0.0;
        self.order_type = OrderCommand::Wait;
        self.order_capital = 0.0;
        self.order_capital_with_leverage = 0.0;
        self.order_created_at = now;
        self.order_quantity = 0.0;
        self.order_fee = 0.0;
        self.order_scanned_at = now;
        self.pnl = 0.0;
        self.roe = 0.0;

        if self.capital <= 85.0 {
            self.is_not_active = true;
        }

        self.in_pos = false;
        Ok(closed_order)
    }

    fn update_statistics(&mut self, pnl: f64) {
        if pnl > 0.0 {
            self.wins += 1;
        } else {
            self.losses += 1;
        }
    }

    pub fn run_strategy(&self, sc: &StrategyContainer) -> (OrderCommand, String) {
        if let Some(s) = &self.strategy {
            return s.run(sc, &self.timeframe, &self.symbol);
        }

        (OrderCommand::Wait, "strategy is none".to_string())
    }

    pub fn is_not_allowed_for_scanning(&self, now: &DateTime<FixedOffset>) -> bool {
        self.is_not_active || self.capital < 85.0 || !is_timeframe_now(self, now.minute()) || self.in_pos
    }
}

fn format_symbol(symbol: &Symbol) -> String {
    let s = format!("{:?}", symbol);
    s.strip_suffix("Usdt").unwrap_or(&s).to_string()
}

impl Debug for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bot")
         .field("name", &self.name)
         .field("capital", &self.capital)
         .finish()
    }
}

impl Clone for Bot {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            timeframe: self.timeframe.clone(),
            strategy_name: self.strategy_name.clone(),
            strategy: None,
            capital: self.capital,
            group: self.group.clone(),
            is_not_active: self.is_not_active,
            wins: self.wins,
            losses: self.losses,
            log: self.log.clone(),
            last_scanned: self.last_scanned,
            started_at: self.started_at,
            leverage: self.leverage,
            take_profit_ratio: self.take_profit_ratio,
            stop_loss_ratio: self.stop_loss_ratio,
            is_trailing_stop_active: self.is_trailing_stop_active,
            trailing_stop_activation_point: self.trailing_stop_activation_point,
            in_pos: self.in_pos,
            order_type: self.order_type,
            order_created_at: self.order_created_at,
            order_scanned_at: self.order_scanned_at,
            order_quantity: self.order_quantity,
            order_capital: self.order_capital,
            order_capital_with_leverage: self.order_capital_with_leverage,
            order_entry_price: self.order_entry_price,
            order_stop_loss: self.order_stop_loss,
            order_take_profit: self.order_take_profit,
            order_fee: self.order_fee,
            pnl: self.pnl,
            roe: self.roe,
        }
    }
}
