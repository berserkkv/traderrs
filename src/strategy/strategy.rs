use crate::enums::{OrderCommand, Symbol, Timeframe};
use crate::models::models::StrategyContainer;
use crate::strategy::str_impl::{EmaBounce, EmaMacd, EmaMacd2};
use std::sync::Arc;

pub trait Strategy {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn run(&self, sc: &StrategyContainer, timeframe: &Timeframe, symbol: &Symbol) -> (OrderCommand, String);
}

pub fn get_strategy(name: &str) -> Arc<dyn Strategy + Send + Sync> {
    match name.to_lowercase().as_str() {
        "emamacd" => Arc::new(EmaMacd {}),
        "emamacd2" => Arc::new(EmaMacd2 {}),
        "emabounce" => Arc::new(EmaBounce {}),
        _ => Arc::new(DummyStrategy {}),
    }
}

#[derive(Debug, Clone)]
pub struct DummyStrategy;
impl Strategy for DummyStrategy {
    fn name(&self) -> &str { "dummy" }
    fn run(&self, _sc: &StrategyContainer, _timeframe: &Timeframe, _symbol: &Symbol) -> (OrderCommand, String) {
        (OrderCommand::Wait, "dummy strategy".to_string())
    }
}