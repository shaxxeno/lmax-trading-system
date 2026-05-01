use chrono::Utc;
use ibapi::Client;
use ibapi::contracts::Contract;
use ibapi::market_data::realtime::{BarSize, WhatToShow};

use super::Feed;
use crate::core::errors::AppError;
use crate::domain::events::market::MarketEvent;
use crate::infrastructure::disruptor::producer::Producer;

pub struct IBFeed {
    symbol: String,
    host: String,
    port: u16,
    client_id: i32,
}

impl IBFeed {
    pub fn new(symbol: String, host: String, port: u16, client_id: i32) -> Self {
        IBFeed { symbol, host, port, client_id }
    }

    fn make_contract(&self) -> Result<Contract, AppError> {
        match self.symbol.as_str() {
            "XAUUSD" => Ok(Contract::futures(&self.symbol)),
            _ => Err(AppError::Feed(format!(
                "Unsupported symbol: {}. Expected only XAUUSD",
                self.symbol
            ))),
        }
    }
}

impl Feed for IBFeed {
    fn stream(&mut self, mut producer: Producer<MarketEvent>) {
        let connection_url = format!("{}:{}", &self.host, &self.port);
        let client = match Client::connect(&connection_url, self.client_id) {
            Ok(c) => c,
            Err(e) => { tracing::error!("IB connect failed: {}", e); return; }
        };
        tracing::info!("Connected to IB Gateway: {}", self.symbol);
        let contract = match self.make_contract() {
            Ok(c) => c,
            Err(e) => { tracing::error!("Contract error: {}", e); return; }
        };
        let bars = match client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::MidPoint, false) {
            Ok(b) => b,
            Err(e) => { tracing::error!("Realtime bars error: {}", e); return; }
        };
        for bar in bars {
            let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
            let event = MarketEvent::new(&self.symbol, bar.close, ts);
            producer.publish(event);
        }
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }
}
