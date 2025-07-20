use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[derive(Eq, Hash, PartialEq, )]
pub enum Symbol {
    SolUsdt,
    BtcUsdt,
}

impl Symbol {
    pub fn to_string(&self) -> String {
        match self {
            Symbol::SolUsdt => String::from("SOLUSDT"),
            Symbol::BtcUsdt => String::from("BTCUSDT"),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Timeframe {
    Min1,
    Min5,
    Min15,
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
        }
    }
}

