use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub enum Symbol {
    SolUsdt,
    BtcUsdt,
    EthUsdt,
    BnbUsdt,
}
impl ToSql for Symbol {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let s = match self {
            Symbol::SolUsdt => "SolUsdt",
            Symbol::BtcUsdt => "BtcUsdt",
            Symbol::EthUsdt => "EthUsdt",
            Symbol::BnbUsdt => "BnbUsdt",
        };

        Ok(rusqlite::types::ToSqlOutput::from(s))
    }
}
impl FromSql for Symbol {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "SolUsdt" => Ok(Symbol::SolUsdt),
            "BtcUsdt" => Ok(Symbol::BtcUsdt),
            "EthUsdt" => Ok(Symbol::EthUsdt),
            "BnbUsdt" => Ok(Symbol::BnbUsdt),
            other => Err(rusqlite::types::FromSqlError::Other(Box::new(
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid Symbol: {}", other))
            ))),

        }
    }
}

impl Symbol {
    pub fn to_string(&self) -> String {
        match self {
            Symbol::SolUsdt => String::from("SOLUSDT"),
            Symbol::BtcUsdt => String::from("BTCUSDT"),
            Symbol::EthUsdt => String::from("ETHUSDT"),
            Symbol::BnbUsdt => String::from("BNBUSDT"),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Timeframe {
    Min1,
    Min5,
    Min15,
    Min30,
    Hour1,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum OrderCommand {
    Long,
    Short,
    Wait,
}
impl OrderCommand {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            OrderCommand::Long => String::from("Long"),
            OrderCommand::Short => String::from("Short"),
            OrderCommand::Wait => String::from("Wait"),
        }
    }
}
impl ToSql for OrderCommand {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
         let s = match self {
             OrderCommand::Long => "Long",
             OrderCommand::Short => "Short",
             OrderCommand::Wait => "Wait",
         };
        Ok(rusqlite::types::ToSqlOutput::from(s))
    }
}

impl FromSql for OrderCommand {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Long" => Ok(OrderCommand::Long),
            "Short" => Ok(OrderCommand::Short),
            "Wait" => Ok(OrderCommand::Wait),
            other => Err(rusqlite::types::FromSqlError::Other(Box::new(
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid OrderCommand: {}", other))
            ))),
        }
    }
}

impl Timeframe {
    pub fn to_string(&self) -> &'static str {
        match self {
            Timeframe::Min1 => "1m",
            Timeframe::Min5 => "5m",
            Timeframe::Min15 => "15m",
            Timeframe::Min30 => "30m",
            Timeframe::Hour1 => "1h",
        }
    }
}
