use crate::domain::entities::trade::{Position, Trade};
pub mod tracker;

pub trait PositionTracker: Send {
    fn open(&mut self, position: Position);
    fn close(&mut self, exit_price: f64) -> Option<Trade>;
    fn current(&self) -> Option<&Position>;
    fn modify_sl(&mut self, new_sl: f64);
    fn close_partial(&mut self, quantity: f64);
    fn pnl(&self, current_price: f64) -> f64;
}
