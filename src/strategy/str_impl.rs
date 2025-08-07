use crate::enums::OrderCommand;
use crate::enums::OrderCommand::{Long, Short, Wait};
use crate::models::models::Candle;
use crate::strategy::strategy::Strategy;
use crate::ta::ema;
use crate::tools::get_close_prices;
use crate::{ta, tools};

pub struct EmaMacd {}
impl Strategy for EmaMacd {
    fn name(&self) -> &str {
        "EmaMacd"
    }

    fn run(&self, candles: &[Candle]) -> (OrderCommand, String) {
        if candles.is_empty() {
            return (Wait, "candles is empty".to_string());
        }

        let closes = tools::get_close_prices(candles);
        let (macd_line, signal_line, histogram) = ta::macd_slice(&closes);
        let ema200 = ta::ema(&closes, 200);

        let n = macd_line.len();
        let macd = macd_line[n - 1];
        let macd_prev = macd_line[n - 2];
        let signal = signal_line[n - 1];
        let hist = histogram[n - 1];
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

pub struct EmaMacd2 {}
impl Strategy for EmaMacd2 {
    fn name(&self) -> &str {
        "EmaMacd2"
    }
    fn run(&self, candles: &[Candle]) -> (OrderCommand, String) {
        if candles.is_empty() {
            return (OrderCommand::Wait, "candles is empty".to_string());
        }

        let closes = tools::get_close_prices(candles);
        let (macd_line, signal_line, _) = ta::macd_slice(&closes);
        let ema200 = ta::ema(&closes, 200);

        let n = macd_line.len();
        let macd = macd_line[n - 1];
        let macd_prev = macd_line[n - 2];
        let signal = signal_line[n - 1];
        let signal_prev = signal_line[n - 2];
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

pub struct EmaBounce {}
impl Strategy for EmaBounce {
    fn name(&self) -> &str {
        "EmaBounce"
    }

    fn run(&self, candles: &[Candle]) -> (OrderCommand, String) {
        if candles.is_empty() {
            return (Wait, "empty candles".to_string());
        }
        let close_price = get_close_prices(candles);
        let ema50 = ema(&close_price, 50);
        let ema200 = ema(&close_price, 200);
        let info = format!("ema50: {:.2}, ema200: {:.2}", ema50, ema200);
        let price = close_price[close_price.len()-1];
        let prev_price = close_price[close_price.len()-2];

        if ema50 > ema200 && prev_price < ema50 && price > ema50 {
            return (Long, info)
        } else if ema50 < ema200 && prev_price > ema50 && price < ema50 {
            return (Short, info)
        }

        (Wait, info)
    }
}






































