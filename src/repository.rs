use crate::models::bot::Bot;
use crate::models::models::{Order, StatisticResult};
use crate::strategy::strategy;
use crate::tools;
use chrono::{DateTime, FixedOffset};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)?;

        conn.execute_batch("
                CREATE TABLE IF NOT EXISTS bot_state (
                    name TEXT PRIMARY KEY,
                    symbol TEXT,
                    timeframe TEXT,
                    strategy_name TEXT,
                    capital INTEGER,
                    bot_group TEXT,
                    is_not_active INTEGER,
                    wins INTEGER,
                    losses INTEGER,
                    log TEXT,
                    started_at TEXT,
                    last_scanned TEXT,
                    leverage INTEGER,
                    take_profit_ratio INTEGER,
                    stop_loss_ratio INTEGER,
                    is_trailing_stop_active bool,
                    trailing_stop_activation_point INTEGER,
                    in_pos INTEGER,
                    order_type TEXT,
                    order_created_at TEXT,
                    order_scanned_at TEXT,
                    order_quantity INTEGER,
                    order_capital INTEGER,
                    order_capital_with_leverage INTEGER,
                    order_entry_price INTEGER,
                    order_stop_loss INTEGER,
                    order_take_profit INTEGER,
                    order_fee INTEGER,
                    pnl INTEGER,
                    roe INTEGER
                );


                CREATE TABLE IF NOT EXISTS bots (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    capital REAL NOT NULL,
                    start_time TEXT NOT NULL,
                    end_time TEXT NOT NULL,
                    wins INTEGER NOT NULL DEFAULT 0,
                    losses INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS orders (
                    id TEXT PRIMARY KEY,
                    symbol TEXT NOT NULL,
                    order_type TEXT NOT NULL,
                    bot_name TEXT NOT NULL,
                    entry_price INTEGER NOT NULL,
                    exit_price INTEGER NOT NULL,
                    quantity INTEGER NOT NULL,
                    pnl INTEGER NOT NULL,
                    roe INTEGER NOT NULL,
                    created_at TEXT NOT NULL,
                    closed_at TEXT NOT NULL,
                    fee INTEGER NOT NULL,
                    leverage INTEGER NOT NULL
                );
            ")?;


        // if !column_names.contains(&"losses".to_string()) {
        //     conn.execute("ALTER TABLE bots ADD COLUMN losses INTEGER NOT NULL", [])?;
        // }
        Ok(Repository { path: db_path })
    }

    pub fn save_bot_state(&self, bots: Vec<Bot>) -> Result<()>{
        let conn = Connection::open(&self.path)?;
        for b in bots {
            conn.execute(
                "INSERT INTO bot_state (
                    name,
                    symbol,
                    timeframe,
                    strategy_name,
                    capital,
                    bot_group,
                    is_not_active,
                    wins,
                    losses,
                    log,
                    started_at,
                    last_scanned,
                    leverage,
                    take_profit_ratio,
                    stop_loss_ratio,
                    is_trailing_stop_active,
                    trailing_stop_activation_point,
                    in_pos,
                    order_type,
                    order_created_at,
                    order_scanned_at,
                    order_quantity,
                    order_capital,
                    order_capital_with_leverage,
                    order_entry_price,
                    order_stop_loss,
                    order_take_profit,
                    order_fee,
                    pnl,
                    roe
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)
                ON CONFLICT(name)
                DO UPDATE SET
                    capital = excluded.capital,
                    is_not_active = excluded.is_not_active,
                    wins = excluded.wins,
                    losses = excluded.losses,
                    log = excluded.log,
                    started_at = excluded.started_at,
                    last_scanned = excluded.last_scanned,
                    leverage = excluded.leverage,
                    take_profit_ratio = excluded.take_profit_ratio,
                    stop_loss_ratio = excluded.stop_loss_ratio,
                    is_trailing_stop_active = excluded.is_trailing_stop_active,
                    trailing_stop_activation_point = excluded.trailing_stop_activation_point,
                    in_pos = excluded.in_pos,
                    order_type = excluded.order_type,
                    order_created_at = excluded.order_created_at,
                    order_scanned_at = excluded.order_scanned_at,
                    order_quantity = excluded.order_quantity,
                    order_capital = excluded.order_capital,
                    order_capital_with_leverage = excluded.order_capital_with_leverage,
                    order_entry_price = excluded.order_entry_price,
                    order_stop_loss = excluded.order_stop_loss,
                    order_take_profit = excluded.order_take_profit,
                    order_fee = excluded.order_fee,
                    pnl = excluded.pnl,
                    roe = excluded.roe;",
                params![b.name, b.symbol, b.timeframe, b.strategy_name, b.capital, b.group, b.is_not_active, b.wins, b.losses, b.log, b.started_at.to_rfc3339(), b.last_scanned.to_rfc3339(), b.leverage, b.take_profit_ratio, b.stop_loss_ratio, b.is_trailing_stop_active, b.trailing_stop_activation_point, b.in_pos, b.order_type, b.order_created_at.to_rfc3339(), b.order_scanned_at.to_rfc3339(), b.order_quantity, b.order_capital, b.order_capital_with_leverage, b.order_entry_price, b.order_stop_loss, b.order_take_profit, b.order_fee, b.pnl, b.roe],
            )?;
        }
        Ok(())
    }

    pub fn get_bot_state(&self) -> Result<Vec<Bot>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT
                    name,
                    symbol,
                    timeframe,
                    strategy_name,
                    capital,
                    bot_group,
                    is_not_active,
                    wins,
                    losses,
                    log,
                    started_at,
                    last_scanned,
                    leverage,
                    take_profit_ratio,
                    stop_loss_ratio,
                    is_trailing_stop_active,
                    trailing_stop_activation_point,
                    in_pos,
                    order_type,
                    order_created_at,
                    order_scanned_at,
                    order_quantity,
                    order_capital,
                    order_capital_with_leverage,
                    order_entry_price,
                    order_stop_loss,
                    order_take_profit,
                    order_fee,
                    pnl,
                    roe
                FROM bot_state")?;

        let bots = stmt.query_map([], |r| {
            let strategy_name: String = r.get(3)?;
            let started_at: String = r.get(10)?;
            let last_scanned: String = r.get(11)?;
            let order_crated_at: String = r.get(19)?;
            let order_scanned_at: String = r.get(20)?;
            let strategy = strategy::get_strategy(&strategy_name);
            Ok(Bot {
                name: r.get(0)?,
                symbol: r.get(1)?,
                timeframe: r.get(2)?,
                strategy_name,
                strategy: Option::from(strategy),
                capital: r.get(4)?,
                group: r.get(5)?,
                is_not_active: r.get(6)?,
                wins: r.get(7)?,
                losses: r.get(8)?,
                log: r.get(9)?,
                started_at: started_at.parse().unwrap(),
                last_scanned: last_scanned.parse().unwrap(),
                leverage: r.get(12)?,
                take_profit_ratio: r.get(13)?,
                stop_loss_ratio: r.get(14)?,
                is_trailing_stop_active: r.get(15)?,
                trailing_stop_activation_point: r.get(16)?,
                in_pos: r.get(17)?,
                order_type: r.get(18)?,
                order_created_at: order_crated_at.parse().unwrap(),
                order_scanned_at: order_scanned_at.parse().unwrap(),
                order_quantity: r.get(21)?,
                order_capital: r.get(22)?,
                order_capital_with_leverage: r.get(23)?,
                order_entry_price: r.get(24)?,
                order_stop_loss: r.get(25)?,
                order_take_profit: r.get(26)?,
                order_fee: r.get(27)?,
                pnl: r.get(28)?,
                roe: r.get(29)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(bots)
    }

    #[allow(dead_code)]
    pub fn create_bot(&self, bot: &Bot) -> Result<usize> {
        let conn = Connection::open(&self.path)?;
        let now = tools::get_date(3);
        let id = format!("{}_{}", bot.name, bot.started_at);
        conn.execute(
            "INSERT INTO bots (id, name, capital, wins, losses, start_time, end_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, bot.name, bot.capital + bot.order_capital, bot.wins, bot.losses, bot.started_at.to_rfc3339(), now.to_rfc3339()],
        )
    }

    pub fn create_bots_in_batch(&self, bots: &mut Vec<Bot>) -> Result<()> {
        let mut conn = Connection::open(&self.path)?;
        let now = tools::get_date(3);

        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare("INSERT INTO bots (id, name, capital, wins, losses, start_time, end_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;

            for b in bots {
                let id = format!("{}_{}", b.name, b.started_at);
                stmt.execute(params![id, b.name, b.capital + b.order_capital, b.wins, b.losses, b.started_at.to_rfc3339(), now.to_rfc3339()])?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_bot(&self, bot_name: String) -> Result<Vec<StatisticResult>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT  name, capital, wins, losses, start_time, end_time FROM bots WHERE name = ?1")?;
        let bots = stmt.query_map([bot_name], |row| {
            let start_time: String = row.get(4)?;
            let end_time: String = row.get(5)?;
            let s = start_time.parse().unwrap();

            Ok(StatisticResult {
                name: row.get(0)?,
                capital: row.get(1)?,
                wins: row.get(2)?,
                losses: row.get(3)?,
                start_time: s,
                end_time: end_time.parse().unwrap(),
            })
        })?
          .collect::<Result<Vec<_>, _>>()?;

        Ok(bots)
    }

    pub fn get_all_bots(&self) -> Result<Vec<StatisticResult>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT name, capital, wins, losses, start_time, end_time FROM bots")?;
        let bots = stmt.query_map([], |row| {
            let start_time: String = row.get(4)?;
            let end_time: String = row.get(5)?;
            let s = start_time.parse().unwrap();

            Ok(StatisticResult {
                name: row.get(0)?,
                capital: row.get(1)?,
                wins: row.get(2)?,
                losses: row.get(3)?,
                start_time: s,
                end_time: end_time.parse().unwrap(),
            })
        })?
          .collect::<Result<Vec<_>, _>>()?;

        Ok(bots)
    }

    pub fn get_orders_in_range(&self, bot_name: String, start_time: DateTime<FixedOffset>, end_time: DateTime<FixedOffset>) -> Result<Vec<Order>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT symbol, order_type, bot_name, entry_price, exit_price, quantity, pnl, roe, created_at, closed_at, fee, leverage FROM orders WHERE bot_name = ?1 AND created_at >= ?2 AND closed_at <= ?3")?;
        let orders = stmt.query_map([bot_name, start_time.to_rfc3339(), end_time.to_rfc3339()], |row| {
            let created_at: String = row.get(8)?;
            let closed_at: String = row.get(9)?;
            Ok(Order {
                symbol: row.get(0)?,
                order_type: row.get(1)?,
                bot_name: row.get(2)?,
                entry_price: row.get(3)?,
                exit_price: row.get(4)?,
                quantity: row.get(5)?,
                pnl: row.get(6)?,
                roe: row.get(7)?,
                created_at: created_at.parse().unwrap(),
                closed_at: closed_at.parse().unwrap(),
                fee: row.get(10)?,
                leverage: row.get(11)?,

            })
        })?
          .collect::<Result<Vec<_>, _>>()?;

        Ok(orders)
    }

    pub fn create_orders(&self, orders: &Vec<Order>) -> Result<()> {
        let mut conn = Connection::open(&self.path)?;


        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare("INSERT INTO orders (id, symbol, order_type, bot_name, entry_price, exit_price, quantity, pnl, roe, created_at, closed_at, fee, leverage) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)")?;

            for o in orders {
                let id = format!("{}_{}", o.bot_name, o.created_at);
                stmt.execute(params![id, o.symbol, o.order_type, o.bot_name, o.entry_price, o.exit_price, o.quantity, o.pnl, o.roe, o.created_at.to_rfc3339(), o.closed_at.to_rfc3339(), o.fee, o.leverage])?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_order_by_bot_name(&self, bot_name: String) -> Result<Vec<Order>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT symbol, order_type, bot_name, entry_price, exit_price, quantity, pnl, roe, created_at, closed_at, fee, leverage FROM orders WHERE bot_name = ?1")?;
        let orders = stmt.query_map([bot_name], |row| {
            let created_at: String = row.get(8)?;
            let closed_at: String = row.get(9)?;
            Ok(Order {
                symbol: row.get(0)?,
                order_type: row.get(1)?,
                bot_name: row.get(2)?,
                entry_price: row.get(3)?,
                exit_price: row.get(4)?,
                quantity: row.get(5)?,
                pnl: row.get(6)?,
                roe: row.get(7)?,
                created_at: created_at.parse().unwrap(),
                closed_at: closed_at.parse().unwrap(),
                fee: row.get(10)?,
                leverage: row.get(11)?,

            })
        })?
          .collect::<Result<Vec<_>, _>>()?;

        Ok(orders)
    }
}

#[allow(dead_code)]
fn drop_table(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(bots)")?;
    let column_names: Vec<String> = stmt.query_map([], |row|
      row.get(1))?
      .collect::<Result<Vec<String>, _>>()?;

    if !column_names.contains(&"wins".to_string()) {
        conn.execute("DROP TABLE IF EXISTS bots", [])?;
    }
    Ok(())
}
