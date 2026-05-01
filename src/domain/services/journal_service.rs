use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

use crate::domain::events::trading::TradingEvent;
use crate::infrastructure::disruptor::consumer::Consumer;

pub struct JournalService {
    consumer: Consumer<TradingEvent>,
    writer: BufWriter<File>,
}

impl JournalService {
    pub fn new(consumer: Consumer<TradingEvent>, path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(JournalService {
            consumer,
            writer: BufWriter::new(file),
        })
    }

    pub fn run(mut self) {
        tracing::info!("JournalService started");
        loop {
            let event = self.consumer.consume();
            match event {
                TradingEvent::Hold => {}
                _ => {
                    let line = self.format(&event);
                    let _ = writeln!(self.writer, "{}", line);
                    let _ = self.writer.flush();
                }
            }
        }
    }

    fn format(&self, event: &TradingEvent) -> String {
        let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        match event {
            TradingEvent::OpenPosition {
                price, quantity, ..
            } => format!("{},OPEN,{:.6},{:.6}", ts, price, quantity),
            TradingEvent::ClosePosition { price, .. } => format!("{},CLOSE,{:.6}", ts, price),
            TradingEvent::ClosePartial { price, quantity, .. } => format!("{},CLOSE_PARTIAL,{:.6},{:.6}", ts, price, quantity),
            TradingEvent::ModifyStopLoss { stop_loss, .. } => format!("{},MODIFY_SL,{:.6}", ts, stop_loss),
            TradingEvent::Hold => String::new(),
        }
    }
}
