use crate::enums::{Symbol, Timeframe};
use crate::models::models::Candle;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

#[derive(Deserialize)]
struct PriceResponse {
    price: String,
}
#[derive(Debug, Clone)]
pub struct BinanceConnector {
    client: Client,
}

impl BinanceConnector {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    pub async fn get_price(&self, symbol: &Symbol) -> Result<f64, Box<dyn Error>> {
        let url = format!(
            "https://fapi.binance.com/fapi/v2/ticker/price?symbol={}",
            symbol.to_string()
        );

        let res = self.client.get(&url).send().await?;
        let price_response: PriceResponse = res.json().await?;
        let price = price_response.price.parse::<f64>()?;

        Ok(price)
    }

    pub async fn get_candles(
        &self,
        symbol: Symbol,
        timeframe: Timeframe,
        limit: i32,
    ) -> Result<Vec<Candle>, Box<dyn Error>> {
        let url = format!(
            "https://fapi.binance.com/fapi/v1/klines?symbol={}&interval={}&limit={}",
            symbol.to_string(),
            timeframe.to_string(),
            limit
        );

        let res = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<Value>>()
            .await?;

        let mut candles: Vec<Candle> = Vec::new();

        for entry in res {
            let open_time = entry[0].as_u64().unwrap_or_default();
            let open = entry[1].as_str().unwrap_or("0").parse::<f64>()?;
            let high = entry[2].as_str().unwrap_or("0").parse::<f64>()?;
            let low = entry[3].as_str().unwrap_or("0").parse::<f64>()?;
            let close = entry[4].as_str().unwrap_or("0").parse::<f64>()?;
            let volume = entry[5].as_str().unwrap_or("0").parse::<f64>()?;

            candles.push(Candle {
                close,
                open,
                high,
                low,
                open_time,
                volume,
            })
        }

        Ok(candles)
    }
}
