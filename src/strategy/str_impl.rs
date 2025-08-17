use crate::enums::OrderCommand::{Long, Short, Wait};
use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::models::StrategyContainer;
use crate::strategy::strategy::Strategy;
use crate::tools::get_close_prices;

#[derive(Debug, Clone)]
pub struct EmaMacd {}
impl Strategy for EmaMacd {
    fn name(&self) -> &str {
        "EmaMacd"
    }

    fn run(&self, sc: &StrategyContainer, timeframe: &Timeframe, symbol: &Symbol) -> (OrderCommand, String) {
        let option_candles = sc.candles_map.get(&(*timeframe, *symbol));
        if option_candles.is_none() {
            return (Wait, "no candles".to_string());
        }

        let candles = option_candles.unwrap();

        if candles.is_empty() {
            return (Wait, "no candles".to_string());
        }

        let macd_data = if let Some(val) = sc.get_macd(timeframe, symbol) {
            val
        } else { return (Wait, "No macd".to_string()) };

        let ema200 = if let Some(val) = sc.get_ema(timeframe, symbol, 200) {
            val[val.len() - 1]
        } else { return (Wait, "No Ema".to_string()) };

        let closes = get_close_prices(candles);


        let n = macd_data.macd.len();
        let macd = macd_data.macd[n - 1];
        let macd_prev = macd_data.macd[n - 2];
        let signal = macd_data.signal[n - 1];
        let hist = macd_data.histogram[n - 1];
        let price = closes[n - 1];

        let info = format!(
            "p:{:.2}, mc:{:.2}, sg:{:.2}, em:{:.2}",
            price, macd, signal, ema200
        );
        
        if macd_prev < 0.0 && macd > 0.0 && hist > 0.0 && price > ema200 {
            (Long, info)
        } else if macd_prev > 0.0 && macd < 0.0 && hist < 0.0 && price < ema200 {
            (Short, info)
        } else {
            (Wait, info)
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmaMacd2 {}
impl Strategy for EmaMacd2 {
    fn name(&self) -> &str {
        "EmaMacd2"
    }
    fn run(&self, sc: &StrategyContainer, timeframe: &Timeframe, symbol: &Symbol) -> (OrderCommand, String) {
        let option_candles = sc.candles_map.get(&(*timeframe, *symbol));
        if option_candles.is_none() {
            return (Wait, "no candles".to_string());
        }

        let candles = option_candles.unwrap();

        if candles.is_empty() {
            return (Wait, "no candles".to_string());
        }

        let macd_data = if let Some(val) = sc.get_macd(timeframe, symbol) {
            val
        } else { return (Wait, "No macd".to_string()) };

        let ema200 = if let Some(val) = sc.get_ema(timeframe, symbol, 200) {
            val[val.len() - 1]
        } else { return (Wait, "No Ema".to_string()) };

        let closes = get_close_prices(candles);

        let n = macd_data.macd.len();
        let macd = macd_data.macd[n - 1];
        let macd_prev = macd_data.macd[n - 2];
        let signal = macd_data.signal[n - 1];
        let signal_prev = macd_data.signal[n - 2];
        let price = closes[n - 1];


        let info = format!(
            "p:{:.2}, mc:{:.2}, sg:{:.2}, em:{:.2}",
            price, macd, signal, ema200
        );

        if macd_prev < signal_prev && macd > signal && price > ema200 {
            (Long, info)
        } else if macd_prev > signal_prev && macd < signal && price < ema200 {
            (Short, info)
        } else {
            (Wait, info)
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmaBounce {}
impl Strategy for EmaBounce {
    fn name(&self) -> &str {
        "EmaBounce"
    }

    fn run(&self, sc: &StrategyContainer, timeframe: &Timeframe, symbol: &Symbol) -> (OrderCommand, String) {
        let option_candles = sc.candles_map.get(&(*timeframe, *symbol));
        if option_candles.is_none() {
            return (Wait, "no candles".to_string());
        }

        let candles = option_candles.unwrap();

        if candles.is_empty() {
            return (Wait, "no candles".to_string());
        }


        let ema200 = if let Some(val) = sc.get_ema(timeframe, symbol, 200) {
            val[val.len() - 1]
        } else { return (Wait, "No Ema 200".to_string()) };

        let ema50 = if let Some(val) = sc.get_ema(timeframe, symbol, 50) {
            val[val.len() - 1]
        } else { return (Wait, "No Ema 50".to_string()) };

        let closes = get_close_prices(candles);


        let n = closes.len();
        let price = closes[n - 1];
        let info = format!("ema50: {:.2}, ema200: {:.2}", ema50, ema200);
        let prev_price = closes[n - 2];

        if ema50 > ema200 && prev_price < ema50 && price > ema50 {
            return (Long, info);
        } else if ema50 < ema200 && prev_price > ema50 && price < ema50 {
            return (Short, info)
        }

        (Wait, info)
    }
}






































