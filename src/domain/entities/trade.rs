use crate::domain::entities::order::OrderSide;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub entry_price: f64,
    pub opened_at: DateTime<Utc>,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub initial_risk: f64,
    pub be_triggered: bool,
    pub partial_triggered: bool,
}

impl Position {
    pub fn pnl(&self, current_price: f64) -> f64 {
        match self.side {
            OrderSide::Long => (current_price - self.entry_price) * self.quantity,
            OrderSide::Short => (self.entry_price - current_price) * self.quantity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
}

impl Trade {
    pub fn pnl(&self) -> f64 {
        match self.side {
            OrderSide::Long => (self.exit_price - self.entry_price) * self.quantity,
            OrderSide::Short => (self.entry_price - self.exit_price) * self.quantity,
        }
    }
}
