pub mod paper;

use crate::core::errors::AppError;
use crate::domain::entities::trade::{Position, Trade};

pub trait Executor: Send {
    fn open(&mut self, symbol: String, price: f64, quantity: f64) -> Result<Position, AppError>;
    fn close(&mut self, price: f64) -> Result<Trade, AppError>;
    fn balance(&self) -> f64;
}
