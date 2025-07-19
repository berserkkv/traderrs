use crate::enums::OrderCommand;
use crate::models::models::Candle;
use crate::strategy::str_impl::{EmaMacd, EmaMacd2};

pub trait Strategy {
    fn name(&self) -> &str;
    fn run(&self, candles: &[Candle]) -> (OrderCommand, String);
}

pub fn get_strategy(name: &str) -> Box<dyn Strategy> {
    match name.to_lowercase().as_str() {
        "emamacd" => Box::new(EmaMacd {}),
        "emamacd2" => Box::new(EmaMacd2 {}),
        _ => Box::new(DummyStrategy {}),
    }
}

pub struct DummyStrategy;
impl Strategy for DummyStrategy {
    fn name(&self) -> &str { "dummy" }
    fn run(&self, _candles: &[Candle]) -> (OrderCommand, String) {
        (OrderCommand::Wait, "dummy strategy".to_string())
    }
}