use std::{io::Read, net::TcpListener};

use crate::{
    domain::events::market::MarketEvent,
    infrastructure::{disruptor::producer::Producer, feed::Feed},
};

pub struct Mt5Feed {
    pub port: u16,
    pub symbol: String,
}

impl Mt5Feed {
    pub fn new(port: u16, symbol: String) -> Self {
        Mt5Feed { port, symbol }
    }
}

impl Feed for Mt5Feed {
    fn stream(&mut self, mut producer: Producer<MarketEvent>) {
        let listener = TcpListener::bind(("0.0.0.0", self.port)).unwrap();
        tracing::info!("Mt5Feed listening on port {}", self.port);

        let (mut stream, addr) = listener.accept().unwrap();
        tracing::info!("MT5 connected from {}", addr);

        let mut buf = [0u8; 16];
        loop {
            stream.read_exact(&mut buf).unwrap();
            let price = f64::from_le_bytes(buf[0..8].try_into().unwrap());
            let timestamp = i64::from_le_bytes(buf[8..16].try_into().unwrap());
            let event = MarketEvent::new(&self.symbol, price, timestamp);
            producer.publish(event);
        }
    }
    fn symbol(&self) -> &str {
        &self.symbol
    }
}
