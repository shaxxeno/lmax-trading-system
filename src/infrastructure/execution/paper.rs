use chrono::Utc;

use crate::core::errors::AppError;
use crate::domain::entities::order::OrderSide;
use crate::domain::entities::trade::{Position, Trade};
use crate::infrastructure::execution::Executor;

pub struct PaperExecutor {
    balance: f64,
    position: Option<Position>,
}

impl PaperExecutor {
    pub fn new(initial_balance: f64) -> Self {
        PaperExecutor {
            balance: initial_balance,
            position: None,
        }
    }
}

impl Executor for PaperExecutor {
    fn open(&mut self, symbol: String, price: f64, quantity: f64) -> Result<Position, AppError> {
        let cost = price * quantity;
        if cost > self.balance {
            return Err(AppError::Execution(format!(
                "Insufficient balance: {:.2}, needed: {:.2}",
                self.balance, cost
            )));
        }

        self.balance -= cost;

        let position = Position {
            symbol,
            side: OrderSide::Long,
            quantity,
            entry_price: price,
            opened_at: Utc::now(),
            stop_loss: 0.0,
            take_profit: 0.0,
            initial_risk: 0.0,
            be_triggered: false,
            partial_triggered: false,
        };

        self.position = Some(position.clone());
        Ok(position)
    }

    fn close(&mut self, price: f64) -> Result<Trade, AppError> {
        let pos = self
            .position
            .take()
            .ok_or_else(|| AppError::Execution("No open position to close".to_string()))?;

        let proceeds = price * pos.quantity;
        self.balance += proceeds;

        let trade = Trade {
            symbol: pos.symbol.clone(),
            side: pos.side.clone(),
            quantity: pos.quantity,
            entry_price: pos.entry_price,
            exit_price: price,
            opened_at: pos.opened_at,
            closed_at: Utc::now(),
        };

        Ok(trade)
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
