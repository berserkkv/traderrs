use crate::calculator::{calculate_pnl, calculate_roe};
use crate::enums::{OrderCommand, Timeframe};
use crate::models::bot::Bot;
use crate::models::models::{BotStatistic, Candle};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use log::debug;
use std::cmp::Ordering;
use std::time::Duration;

pub fn get_close_prices(candles: &[Candle]) -> Vec<f64> {
    let mut close_prices: Vec<f64> = Vec::with_capacity(candles.len());
    for c in candles {
        close_prices.push(c.close);
    }
    close_prices
}

#[allow(dead_code)]
pub fn get_open_prices(candles: &[Candle]) -> Vec<f64> {
    let mut prices: Vec<f64> = Vec::with_capacity(candles.len());
    for c in candles {
        prices.push(c.open);
    }
    prices
}

#[allow(dead_code)]
pub fn get_high_prices(candles: &[Candle]) -> Vec<f64> {
    let mut prices: Vec<f64> = Vec::with_capacity(candles.len());
    for c in candles {
        prices.push(c.high);
    }
    prices
}


#[allow(dead_code)]
pub fn get_low_prices(candles: &[Candle]) -> Vec<f64> {
    let mut prices: Vec<f64> = Vec::with_capacity(candles.len());
    for c in candles {
        prices.push(c.low);
    }
    prices
}


#[allow(dead_code)]
pub fn get_volume(candles: &[Candle]) -> Vec<f64> {
    let mut vol: Vec<f64> = Vec::with_capacity(candles.len());
    for c in candles {
        vol.push(c.volume);
    }
    vol
}
pub fn format_timeframe(timeframe: &Timeframe) -> String {
    match timeframe {
        Timeframe::Min1 => "1m".to_string(),
        Timeframe::Min5 => "5m".to_string(),
        Timeframe::Min15 => "15m".to_string(),
        Timeframe::Min30 => "30m".to_string(),
        Timeframe::Hour1 => "1h".to_string(),
    }
}
pub fn should_close_position(price: f64, bot: &Bot) -> bool {
    match bot.order_type {
        OrderCommand::Long => price <= bot.order_stop_loss || price >= bot.order_take_profit,
        OrderCommand::Short => price >= bot.order_stop_loss || price <= bot.order_take_profit,
        _ => false,
    }
}
pub fn update_pnl_and_roe(bot: &mut Bot, price: f64) {
    bot.pnl = calculate_pnl(
        price,
        bot.order_capital_with_leverage,
        bot.order_quantity,
        &bot.order_type,
    );
    bot.roe = calculate_roe(bot.order_entry_price, price, bot.leverage, &bot.order_type);
}
pub fn is_timeframe_now(bot: &Bot, minute: u32) -> bool {
    match bot.timeframe {
        Timeframe::Min1 => true,
        Timeframe::Min5 => minute % 5 == 0,
        Timeframe::Min15 => minute % 15 == 0,
        Timeframe::Min30 => minute % 30 == 0,
        Timeframe::Hour1 => minute % 60 == 0,
    }
}
pub fn shift_stop_loss(bot: &mut Bot) {
    if !bot.is_trailing_stop_active {
        return;
    }

    let real_roe = bot.roe / bot.leverage;

    if real_roe <= bot.trailing_stop_activation_point {
        return;
    }

    let pnl_decimal = real_roe / 100.0;
    let mut shift = pnl_decimal / 2.0;

    if real_roe < bot.trailing_stop_activation_point {
        shift = 0.0;
    }
    let new_stop_loss: f64;
    if bot.order_type == OrderCommand::Long {
        new_stop_loss = bot.order_entry_price * (1.0 + shift);
        if new_stop_loss > bot.order_stop_loss {
            debug!("Stop loss shifted, new stop loss: {}", new_stop_loss);
            bot.order_stop_loss = new_stop_loss;
        }
    } else if bot.order_type == OrderCommand::Short {
        new_stop_loss = bot.order_entry_price * (1.0 - shift);
        if new_stop_loss < bot.order_stop_loss {
            debug!("Stop loss shifted, new stop loss: {}", new_stop_loss);
            bot.order_stop_loss = new_stop_loss;
        }
    }
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

pub fn sort_bots(bots: &mut Vec<Bot>) -> &mut Vec<Bot> {
    bots.sort_by(|a, b| {
        a.is_not_active
            .cmp(&b.is_not_active)
            .then(cmp_f64(
                &(b.capital + b.order_capital),
                &(a.capital + a.order_capital),
            ))
            .then(a.timeframe.cmp(&b.timeframe))
    });
    bots
}

pub fn sort_bot_statistics(bot_statisitcs: &mut Vec<BotStatistic>) -> &mut Vec<BotStatistic> {
    bot_statisitcs.sort_by(|a, b| {
        cmp_f64(&b.capital, &a.capital)
          .then(a.bot_name.cmp(&b.bot_name))
    });
    bot_statisitcs
}

pub fn get_date(time_zone: i32) -> DateTime<FixedOffset> {
    return Utc::now().with_timezone(&FixedOffset::east_opt(time_zone * 3600).unwrap());
}

pub fn parse_time(time_str: &str) -> DateTime<FixedOffset> {
    // Parse without timezone
    let naive = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M")
      .expect("Failed to parse time");

    // Istanbul is UTC+3
    let ist_offset = FixedOffset::east_opt(3 * 3600).unwrap(); // 3 hours in seconds

    // Convert to DateTime<FixedOffset>
    DateTime::<FixedOffset>::from_utc(naive, ist_offset)
}

fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => a.partial_cmp(b).unwrap(),
    }
}

