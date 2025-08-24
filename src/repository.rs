use crate::models::bot::Bot;
use crate::models::models::{Order, StatisticResult};
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
                DROP TABLE IF EXISTS bots;
                DROP TABLE IF EXISTS orders;

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

    pub fn create_bot(&self, bot: &Bot) -> Result<usize> {
        let conn = Connection::open(&self.path)?;
        let now = tools::get_date(3);
        let id = format!("{}_{}", bot.name, bot.started_at);
        conn.execute(
            "INSERT INTO bots (id, name, capital, wins, losses, start_time, end_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, bot.name, bot.capital + bot.order_capital, bot.wins, bot.losses, bot.started_at.to_rfc3339(), now.to_rfc3339()],
        )
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
fn drop_table(conn: &Connection) ->Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(bots)")?;
    let column_names: Vec<String> = stmt.query_map([], |row|
      row.get(1))?
      .collect::<Result<Vec<String>, _>>()?;

    if !column_names.contains(&"wins".to_string()) {
        conn.execute("DROP TABLE IF EXISTS bots", [])?;
    }
    Ok(())
}
