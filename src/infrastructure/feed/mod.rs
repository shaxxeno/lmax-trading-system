pub mod binance;
pub mod ib;
pub mod mt5;

use crate::domain::events::market::MarketEvent;
use crate::infrastructure::disruptor::producer::Producer;

pub trait Feed: Send {
    fn stream(&mut self, producer: Producer<MarketEvent>);
    fn symbol(&self) -> &str;
}

pub struct NoopFeed;

impl Feed for NoopFeed {
    fn stream(&mut self, _producer: Producer<MarketEvent>) {}
    fn symbol(&self) -> &str {
        ""
    }
}
