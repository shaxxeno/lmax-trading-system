use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceEvent {
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub symbol: String,
    pub last: Option<f64>,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        OrderBook { symbol, last: None }
    }
    pub fn update(&mut self, event: PriceEvent) {
        self.last = Some(event.price)
    }
}
