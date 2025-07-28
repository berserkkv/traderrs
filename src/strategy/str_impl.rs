use crate::enums::OrderCommand;
use crate::models::models::Candle;
use crate::strategy::strategy::Strategy;
use crate::{ta, tools};

pub struct EmaMacd {}
impl Strategy for EmaMacd {
    fn name(&self) -> &str {
        "EmaMacd"
    }

    fn run(&self, candles: &[Candle]) -> (OrderCommand, String) {
        if candles.is_empty() {
            return (OrderCommand::Wait, "candles is empty".to_string());
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
            (OrderCommand::Long, info)
        } else if macd_prev > 0.0 && macd < 0.0 && hist < 0.0 && price < ema200 {
            (OrderCommand::Short, info)
        } else {
            (OrderCommand::Wait, info)
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
            (OrderCommand::Long, info)
        } else if macd_prev > signal_prev && macd < signal && price < ema200 {
            (OrderCommand::Short, info)
        } else {
            (OrderCommand::Wait, info)
        }
    }
}
