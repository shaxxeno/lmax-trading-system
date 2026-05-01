use crate::domain::entities::trade::Signal;
pub mod breakout;

pub trait Strategy: Send {
    fn name(&self) -> &str;
    fn on_price(&mut self, price: f64, timestamp: i64) -> Signal;
}
