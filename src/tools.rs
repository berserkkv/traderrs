use crate::calculator::{calculate_pnl, calculate_roe};
use crate::enums::{OrderCommand, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::Candle;
use std::time::Duration;

pub fn get_close_prices(candles: &[Candle]) -> Vec<f64> {
    let mut close_prices: Vec<f64> = Vec::with_capacity(candles.len());

    for i in 0..candles.len() {
        close_prices[i] = candles[i].close;
    }
    close_prices
}
pub fn format_timeframe(timeframe: &Timeframe) -> String {
    match timeframe {
        Timeframe::Min1 => { "1m".to_string() }
        Timeframe::Min5 => { "5m".to_string() }
        Timeframe::Min15 => { "15m".to_string() }
    }
}


pub fn should_close_position(price: f64, bot: &Bot) -> bool {
    match bot.order_type {
        OrderCommand::Long => price <= bot.order_stop_loss || price >= bot.order_take_profit,
        OrderCommand::Short => price >= bot.order_stop_loss || price <= bot.order_take_profit,
        _ => true,
    }
}

pub fn update_pnl_and_roe(bot: &mut Bot, price: f64) {
    bot.pnl = calculate_pnl(price, bot.order_capital_with_leverage, bot.order_quantity, &bot.order_type);
    bot.roe = calculate_roe(bot.order_entry_price, price, bot.leverage, &bot.order_type);
}

pub fn shift_stop_loss(_bot: &mut Bot) {
    // TODO: implement trailing stop logic here
}

pub async fn wait_until_next_aligned_tick(interval: Duration) {
    use std::time::SystemTime;

    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let elapsed_ns = since_epoch.as_nanos() % interval.as_nanos();
    let wait_ns = if elapsed_ns == 0 {
        interval.as_nanos()
    } else {
        interval.as_nanos() - elapsed_ns
    };

    let wait_duration = Duration::from_nanos(wait_ns as u64);
    tokio::time::sleep(wait_duration).await;
}