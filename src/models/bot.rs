use crate::binance_connector::BinanceConnector;
use crate::calculator::{
  calculate_buy_quantity, calculate_maker_fee, calculate_pnl, calculate_roe, calculate_stop_loss,
  calculate_take_profit, calculate_taker_fee,
};
use crate::enums::Symbol::SolUsdt;
use crate::enums::Timeframe::Min1;
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::models::Order;
use crate::tools;
use chrono::{DateTime, FixedOffset, Utc};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
  pub id: i64,
  pub name: String,
  pub symbol: Symbol,
  pub timeframe: Timeframe,
  pub strategy_name: String,
  pub capital: f64,
  pub group: String,
  pub is_not_active: bool,

  pub wins: i16,
  pub losses: i16,

  pub log: String,
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
    let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
    let now = Utc::now().with_timezone(&offset);

    Self {
      id: generate_bot_id(),
      name,
      symbol,
      is_not_active: false,
      timeframe,
      strategy_name,
      group: format!("{:?}{:?}", timeframe, symbol),
      capital,
      last_scanned: now,
      log: "".to_string(),

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

  pub fn new_dummy() -> Self {
    let name = format!("{}", "Dummy");
    let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
    let now = Utc::now().with_timezone(&offset);

    Self {
      id: 1000,
      name,
      symbol: SolUsdt,
      is_not_active: false,
      timeframe: Min1,
      strategy_name: "Macd".to_string(),
      capital: 1000.0,
      last_scanned: now,
      log: "".to_string(),
      group: "".to_string(),

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

  pub fn can_open_position(&self) -> Result<(), Box<dyn Error>> {
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

  pub async fn open_position(
    &mut self,
    command: &OrderCommand,
    connector: &BinanceConnector,
  ) -> Result<(), Box<dyn Error>> {
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

    let offset = FixedOffset::east_opt(3 * 60 * 60).unwrap(); // +3 utc
    let now = Utc::now().with_timezone(&offset);

    self.order_capital_with_leverage = self.leverage * capital;
    self.order_capital = capital;
    self.order_quantity = calculate_buy_quantity(price, self.order_capital_with_leverage);
    self.order_entry_price = price;
    self.in_pos = true;
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
      bot_id: self.id,
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
    self.in_pos = false;
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

    Ok(closed_order)
  }

  fn update_statistics(&mut self, pnl: f64) {
    if pnl > 0.0 {
      self.wins += 1;
    } else {
      self.losses += 1;
    }
  }
}

fn format_symbol(symbol: &Symbol) -> String {
  let s = format!("{:?}", symbol);
  s.strip_suffix("Usdt").unwrap_or(&s).to_string()
}

static BOT_ID_COUNTER: AtomicI64 = AtomicI64::new(1);

fn generate_bot_id() -> i64 {
  BOT_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
