use std::sync::Arc;
use std::time::Instant;

use chrono::Utc;

use crate::{
    core::{config::Settings, metrics::LatencyStats},
    domain::{
        entities::{
            order::OrderSide,
            trade::{Position, Signal},
        },
        events::{market::MarketEvent, trading::TradingEvent},
        services::{position::PositionTracker, risk::RiskManager, strategies::Strategy},
    },
    infrastructure::disruptor::{consumer::Consumer, producer::Producer},
};

pub struct BusinessProcessor<S: Strategy> {
    inbound: Consumer<MarketEvent>,
    outbound: Producer<TradingEvent>,
    strategy: S,
    risk: Box<dyn RiskManager>,
    tracker: Box<dyn PositionTracker>,
    settings: Arc<Settings>,
    metrics: LatencyStats,
}

impl<S: Strategy> BusinessProcessor<S> {
    pub fn new(
        inbound: Consumer<MarketEvent>,
        outbound: Producer<TradingEvent>,
        strategy: S,
        risk: Box<dyn RiskManager>,
        tracker: Box<dyn PositionTracker>,
        settings: Arc<Settings>,
    ) -> Self {
        BusinessProcessor {
            inbound,
            outbound,
            strategy,
            risk,
            tracker,
            settings,
            metrics: LatencyStats::new(),
        }
    }

    pub fn run(mut self) {
        tracing::info!("BLP started");
        loop {
            let event = self.inbound.consume();
            let t0 = Instant::now();

            let balance = self.settings.trading.initial_balance;
            let quantity = self.risk.max_position_size(balance, event.price);
            let sig = self.strategy.on_price(event.price, event.timestamp);
            let trading_event = self.translate(sig, event, quantity);
            self.outbound.publish(trading_event);

            self.metrics.record(t0.elapsed().as_nanos() as u64);
        }
    }

    fn translate(&mut self, sig: Signal, event: MarketEvent, quantity: f64) -> TradingEvent {
        match sig {
            Signal::Buy if self.tracker.current().is_none() => {
                let sym = std::str::from_utf8(&event.symbol[..event.sym_len as usize])
                    .unwrap_or("")
                    .to_string();
                self.tracker.open(Position {
                    symbol: sym,
                    side: OrderSide::Long,
                    quantity,
                    entry_price: event.price,
                    opened_at: Utc::now(),
                    stop_loss: 0.0,
                    take_profit: 0.0,
                    initial_risk: 0.0,
                    be_triggered: false,
                    partial_triggered: false,
                });
                TradingEvent::OpenPosition {
                    symbol: event.symbol,
                    sym_len: event.sym_len,
                    price: event.price,
                    quantity,
                    timestamp: event.timestamp,
                }
            }
            Signal::Sell if self.tracker.current().is_some() => {
                self.tracker.close(event.price);
                TradingEvent::ClosePosition {
                    symbol: event.symbol,
                    sym_len: event.sym_len,
                    price: event.price,
                    timestamp: event.timestamp,
                }
            }
            _ => TradingEvent::Hold,
        }
    }
}
