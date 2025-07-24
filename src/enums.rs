use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub enum Symbol {
    SolUsdt,
    BtcUsdt,
    EthUsdt,
    BnbUsdt,
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
    Hour1,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum OrderCommand {
    Long,
    Short,
    Wait,
}

impl Timeframe {
    pub fn to_string(&self) -> &'static str {
        match self {
            Timeframe::Min1 => "1m",
            Timeframe::Min5 => "5m",
            Timeframe::Min15 => "15m",
            Timeframe::Hour1 => "1h",
        }
    }
}
