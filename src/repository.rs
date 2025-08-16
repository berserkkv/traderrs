use crate::models::bot::Bot;
use crate::models::models::StatisticResult;
use crate::tools;
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use tokio::sync::RwLockReadGuard;

#[derive(Debug, Clone)]
pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("
                CREATE TABLE IF NOT EXISTS bots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    capital REAL NOT NULL,
                    start_time TEXT NOT NULL,
                    end_time TEXT NOT NULL,
                    wins INTEGER NOT NULL DEFAULT 0,
                    losses INTEGER NOT NULL DEFAULT 0
                );

            ")?;

        // let mut stmt = conn.prepare("PRAGMA table_info(bots)")?;
        // let column_names: Vec<String> = stmt.query_map([], |row|
        // row.get(1))?
        //   .collect::<Result<Vec<String>, _>>()?;
        //
        // if !column_names.contains(&"wins".to_string()) {
        //     conn.execute("ALTER TABLE bots ADD COLUMN wins INTEGER NOT NULL", [])?;
        // }
        //
        // if !column_names.contains(&"losses".to_string()) {
        //     conn.execute("ALTER TABLE bots ADD COLUMN losses INTEGER NOT NULL", [])?;
        // }
        Ok(Repository { path: db_path })
    }

    pub fn create_bot(&self, bot: RwLockReadGuard<Bot>) -> Result<usize> {
        let conn = Connection::open(&self.path)?;
        let now = tools::get_date(3);
        conn.execute(
            "INSERT INTO bots (name, capital, wins, losses, start_time, end_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![bot.name, bot.capital + bot.order_capital, bot.wins, bot.losses, bot.started_at.to_rfc3339(), now.to_rfc3339()],
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
}
