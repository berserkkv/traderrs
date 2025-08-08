use crate::models::bot::Bot;
use crate::models::models::BotDto;
use chrono::{DateTime, FixedOffset, Local};
use rusqlite::{params, Connection, Result};
use tokio::sync::RwLockReadGuard;

#[derive(Debug, Clone)]
pub struct Repository {
    path: String,
}
impl Repository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS bots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            capital REAL NOT NULL,
            created_at TEXT NOT NULL);
        ")?;
        Ok(Repository { path: db_path.to_string() })
    }

    pub fn create_bot(&self, bot: RwLockReadGuard<Bot>) -> Result<usize> {
        let conn = Connection::open(&self.path)?;
        let now = Local::now().with_timezone(&FixedOffset::east_opt(3 * 60 * 60).unwrap()).to_rfc3339();
        conn.execute(
            "INSERT INTO bots (name, capital, created_at) VALUES (?1, ?2, ?3)", params![bot.name, bot.capital + bot.order_capital, now],
        )
    }

    // pub fn get_bot(&self, bot_name: String) -> Result<Option<BotDto>> {
    //     let mut stmt = self.conn.prepare("SELECT id, name, capital FROM bots WHERE name = ?1")?;
    //     let mut rows = stmt.query(params![bot_name])?;
    //
    //     if let Some(row) = rows.next()? {
    //         Ok(Some(BotDto {
    //             name: row.get(1)?,
    //             capital: row.get(2)?,
    //         }))
    //     } else {
    //         Ok(None)
    //     }
    // }

    pub fn get_all_bots(&self) -> Result<Vec<BotDto>> {
        let conn = Connection::open(&self.path)?;
        let mut stmt = conn.prepare("SELECT name, capital, created_at FROM bots")?;
        let bots = stmt.query_map([], |row| {
            let created_at: String = row.get(2)?;
            Ok(BotDto {
                name: row.get(0)?,
                capital: row.get(1)?,
                created_at: DateTime::parse_from_rfc3339(&created_at).unwrap(),
            })
        })?
          .collect::<Result<Vec<_>, _>>()?;

        Ok(bots)
    }
}
