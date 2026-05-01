use crate::core::errors::AppError;
pub mod basic;

pub trait RiskManager: Send {
    fn check_order(&self, price: f64, quantity: f64) -> Result<(), AppError>;
    fn check_drawdown(&self, current_balance: f64, initial_balance: f64) -> Result<(), AppError>;
    fn max_position_size(&self, balance: f64, price: f64) -> f64;
}
