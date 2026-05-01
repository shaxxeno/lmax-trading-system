use crate::domain::entities::trade::{Position, Trade};
use crate::domain::services::position::PositionTracker;
use chrono::Utc;

pub struct BasicPositionTracker {
    position: Option<Position>,
    trades: Vec<Trade>,
}

impl BasicPositionTracker {
    pub fn new() -> Self {
        BasicPositionTracker {
            position: None,
            trades: Vec::new(),
        }
    }

    pub fn total_pnl(&self) -> f64 {
        self.trades.iter().map(|t| t.pnl()).sum()
    }

    pub fn trade_count(&self) -> usize {
        self.trades.len()
    }

    pub fn win_rate(&self) -> f64 {
        if self.trades.is_empty() {
            return 0.0;
        }
        let wins = self.trades.iter().filter(|t| t.pnl() > 0.0).count();
        wins as f64 / self.trades.len() as f64 * 100.0
    }
}

impl PositionTracker for BasicPositionTracker {
    fn open(&mut self, position: Position) {
        self.position = Some(position);
    }

    fn close(&mut self, exit_price: f64) -> Option<Trade> {
        if let Some(pos) = self.position.take() {
            let trade = Trade {
                symbol: pos.symbol.clone(),
                side: pos.side.clone(),
                quantity: pos.quantity,
                entry_price: pos.entry_price,
                exit_price,
                opened_at: pos.opened_at,
                closed_at: Utc::now(),
            };
            self.trades.push(trade.clone());
            return Some(trade);
        }
        None
    }
    fn current(&self) -> Option<&Position> {
        self.position.as_ref()
    }

    fn modify_sl(&mut self, new_sl: f64) {
        if let Some(p) = self.position.as_mut() {
            p.stop_loss = new_sl;
        }
    }

    fn close_partial(&mut self, quantity: f64) {
        if let Some(p) = self.position.as_mut() {
            p.quantity -= quantity;
        }
    }

    fn pnl(&self, current_price: f64) -> f64 {
        self.position
            .as_ref()
            .map(|p| p.pnl(current_price))
            .unwrap_or(0.0)
    }
}
