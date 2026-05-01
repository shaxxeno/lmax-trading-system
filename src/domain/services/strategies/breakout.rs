use crate::domain::entities::trade::Signal;
use crate::domain::services::strategies::Strategy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Session {
    Asia,
    London,
    NewYork,
    Off,
}

pub fn detect_session(timestamp: i64) -> Session {
    let secs = (timestamp / 1_000_000_000) % 86400;
    match secs {
        s if s < 8 * 3600 => Session::Asia,
        s if s < 17 * 3600 => Session::London,
        s if s >= 13 * 3600 && s < 22 * 3600 => Session::NewYork,
        _ => Session::Off,
    }
}

pub struct AsiaRange {
    pub high: f64,
    pub low: f64,
    pub set: bool,
}

impl AsiaRange {
    pub fn new() -> Self {
        AsiaRange {
            high: f64::MIN,
            low: f64::MAX,
            set: false,
        }
    }

    pub fn update(&mut self, price: f64) {
        if price > self.high {
            self.high = price;
        }
        if price < self.low {
            self.low = price;
        }
        self.set = true;
    }

    pub fn reset(&mut self) {
        self.high = f64::MIN;
        self.low = f64::MAX;
        self.set = false;
    }
}

pub struct BreakoutStrategy {
    asia_range: AsiaRange,
    last_session: Session,
    tick_count: u64,
}

impl BreakoutStrategy {
    pub fn new() -> Self {
        BreakoutStrategy {
            asia_range: AsiaRange::new(),
            last_session: Session::Off,
            tick_count: 0,
        }
    }
}

impl Strategy for BreakoutStrategy {
    fn name(&self) -> &str {
        "asia_range_breakout"
    }

    fn on_price(&mut self, price: f64, timestamp: i64) -> Signal {
        self.tick_count += 1;
        if self.tick_count == 1 {
            return Signal::Buy;
        }

        let session = detect_session(timestamp);

        if session == Session::Asia && self.last_session != Session::Asia {
            self.asia_range.reset();
        }
        if session == Session::Asia {
            self.asia_range.update(price);
        }

        let signal = if session == Session::London && self.asia_range.set {
            if price > self.asia_range.high {
                Signal::Buy
            } else if price < self.asia_range.low {
                Signal::Sell
            } else {
                Signal::Hold
            }
        } else {
            Signal::Hold
        };

        self.last_session = session;
        signal
    }
}
