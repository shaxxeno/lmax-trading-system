#[derive(Debug, Clone, Copy)]
pub struct MarketEvent {
    pub symbol: [u8; 16], // fixed array, no heap allocation
    pub sym_len: u8,
    pub price: f64,
    pub timestamp: i64, //unix nanoseconds, chrono::DateTime is not Copy
}

impl MarketEvent {
    pub fn new(symbol: &str, price: f64, timestamp: i64) -> Self {
        let mut buf = [0u8; 16];
        let bytes = symbol.as_bytes();
        let len = bytes.len().min(16);
        buf[..len].copy_from_slice(&bytes[..len]);
        MarketEvent {
            symbol: buf,
            sym_len: len as u8,
            price,
            timestamp,
        }
    }

    pub fn symbol_str(&self) -> &str {
        let len = self.sym_len as usize;
        std::str::from_utf8(&self.symbol[..len]).unwrap_or("")
    }
}

impl Default for MarketEvent {
    fn default() -> Self {
        MarketEvent {
            symbol: [0u8; 16],
            sym_len: 0,
            price: 0.0,
            timestamp: 0,
        }
    }
}
