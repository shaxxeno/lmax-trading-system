use crate::{
    domain::events::trading::TradingEvent,
    infrastructure::{
        disruptor::consumer::Consumer,
        execution::{Executor, paper::PaperExecutor},
    },
};

pub struct OrderService {
    consumer: Consumer<TradingEvent>,
    executor: PaperExecutor,
}

impl OrderService {
    pub fn new(consumer: Consumer<TradingEvent>, executor: PaperExecutor) -> Self {
        OrderService { consumer, executor }
    }

    pub fn run(mut self) {
        tracing::info!("OrderService started");
        loop {
            let event = self.consumer.consume();
            match event {
                TradingEvent::OpenPosition {
                    symbol,
                    sym_len,
                    price,
                    quantity,
                    ..
                } => {
                    let sym = std::str::from_utf8(&symbol[..sym_len as usize]).unwrap_or("");
                    match self.executor.open(sym.to_string(), price, quantity) {
                        Ok(_) => tracing::info!(
                            "Opened position: {} @ {:.2} qty={:.4}",
                            sym,
                            price,
                            quantity
                        ),
                        Err(e) => tracing::error!("Open failed: {}", e),
                    }
                }
                TradingEvent::ClosePosition {
                    symbol,
                    sym_len,
                    price,
                    ..
                } => {
                    let sym = std::str::from_utf8(&symbol[..sym_len as usize]).unwrap_or("");
                    match self.executor.close(price) {
                        Ok(trade) => tracing::info!(
                            "Closed position: {} @ {:.2} pnl={:.2}",
                            sym,
                            price,
                            trade.pnl()
                        ),
                        Err(e) => tracing::error!("Close failed: {}", e),
                    }
                }
                TradingEvent::ClosePartial { symbol, sym_len, price, quantity, .. } => {
                    let sym = std::str::from_utf8(&symbol[..sym_len as usize]).unwrap_or("");
                    tracing::info!("Partial close: {} @ {:.2} qty={:.4}", sym, price, quantity);
                }
                TradingEvent::ModifyStopLoss { symbol, sym_len, stop_loss, .. } => {
                    let sym = std::str::from_utf8(&symbol[..sym_len as usize]).unwrap_or("");
                    tracing::info!("SL modified: {} new_sl={:.6}", sym, stop_loss);
                }
                TradingEvent::Hold => {}
            }
        }
    }
}
