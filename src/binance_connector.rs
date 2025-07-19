use crate::enums::{Symbol, Timeframe};
use crate::models::models::Candle;
use serde::Deserialize;

#[derive(Deserialize)]
struct PriceResponse {
    price: String,
}
#[derive(Debug, Clone)]
pub struct BinanceConnector {
    // client: Client,
}

impl BinanceConnector {
    pub fn new() -> Self {
        Self {
            // client: Client::builder()
            //     .timeout(std::time::Duration::from_secs(10))
            //     .build()
            //     .unwrap(),
        }
    }

    pub fn get_price(&self, symbol: &Symbol) -> f64 {
        // let url = format!(
        //     "https://fapi.binance.com/fapi/v2/ticker/price?symbol={}",
        //     symbol.to_string().to_uppercase()
        // );
        //
        // let resp = self.client.get(&url).send();
        // match resp {
        //     Ok(r) => {
        //         let pr: Result<PriceResponse, _> = r.json();
        //         match pr {
        //             Ok(price) => price.price.parse().unwrap_or(0.0),
        //             Err(_) => 0.0,
        //         }
        //     }
        //     Err(_) => 0.0,
        // }
        0.0
    }

    pub(crate) fn get_candles(&self, p0: Symbol, p1: Timeframe, p2: i32) -> Vec<Candle> {
        Vec::new()
    }
}
