use crate::core::errors::AppError;
use crate::domain::services::risk::RiskManager;

pub struct BasicRiskManager {
    max_position_pct: f64,
    max_drawdown_pct: f64,
    initial_balance: f64,
}

impl BasicRiskManager {
    pub fn new(max_position_pct: f64, max_drawdown_pct: f64, initial_balance: f64) -> Self {
        BasicRiskManager {
            max_position_pct,
            max_drawdown_pct,
            initial_balance,
        }
    }
}

impl RiskManager for BasicRiskManager {
    fn check_order(&self, price: f64, quantity: f64) -> Result<(), AppError> {
        let order_value = price * quantity;
        let max_value = self.initial_balance * self.max_position_pct;

        if order_value > max_value {
            return Err(AppError::Risk(format!(
                "Order value {:.2} exceeds max allowed {:.2}",
                order_value, max_value
            )));
        }
        Ok(())
    }

    fn check_drawdown(&self, current_balance: f64, initial_balance: f64) -> Result<(), AppError> {
        let drawdown = (initial_balance - current_balance) / initial_balance;

        if drawdown > self.max_drawdown_pct {
            return Err(AppError::Risk(format!(
                "Drawdown {:.2}% exceeds max allowed {:.2}%",
                drawdown * 100.0,
                self.max_drawdown_pct * 100.0
            )));
        }
        Ok(())
    }
    fn max_position_size(&self, balance: f64, price: f64) -> f64 {
        (balance * self.max_position_pct) / price
    }
}
