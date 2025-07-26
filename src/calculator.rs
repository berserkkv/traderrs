use crate::enums::OrderCommand;

pub fn calculate_stop_loss(price: f64, stop_loss_pct: f64, order_type: &OrderCommand) -> f64 {
    match order_type {
        OrderCommand::Long => price * (1.0 - stop_loss_pct / 100.0),
        OrderCommand::Short => price * (1.0 + stop_loss_pct / 100.0),
        _ => 0.0,
    }
}

pub fn calculate_take_profit(price: f64, take_profit_pct: f64, order_type: &OrderCommand) -> f64 {
    match order_type {
        OrderCommand::Long => price * (1.0 + take_profit_pct / 100.0),
        OrderCommand::Short => price * (1.0 - take_profit_pct / 100.0),
        _ => 0.0,
    }
}

pub fn calculate_taker_fee(capital: f64) -> f64 {
    // e.g. 0.1%
    capital * 0.0004
}

pub fn calculate_maker_fee(capital: f64) -> f64 {
    // e.g. 0.05%
    capital * 0.0002
}

pub fn calculate_buy_quantity(price: f64, capital_with_leverage: f64) -> f64 {
    if price == 0.0 {
        return 0.0;
    }
    capital_with_leverage / price
}

pub fn calculate_pnl(
    current_price: f64,
    capital_with_leverage: f64,
    quantity: f64,
    order_type: &OrderCommand,
) -> f64 {
    if quantity == 0.0 {
        return 0.0;
    }
    match order_type {
        OrderCommand::Long => (current_price - capital_with_leverage / quantity) * quantity,
        OrderCommand::Short => (capital_with_leverage / quantity - current_price) * quantity,
        _ => 0.0,
    }
}

pub fn calculate_roe(
    entry_price: f64,
    current_price: f64,
    leverage: f64,
    order_type: &OrderCommand,
) -> f64 {
    if entry_price == 0.0 {
        return 0.0;
    }
    match order_type {
        OrderCommand::Long => ((current_price - entry_price) / entry_price) * leverage * 100.0,
        OrderCommand::Short => ((entry_price - current_price) / entry_price) * leverage * 100.0,
        _ => 0.0,
    }
}
