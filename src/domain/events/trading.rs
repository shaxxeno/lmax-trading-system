#[derive(Debug, Clone, Copy)]
pub enum TradingEvent {
    OpenPosition {
        symbol: [u8; 16],
        sym_len: u8,
        price: f64,
        quantity: f64,
        timestamp: i64,
    },
    ClosePosition {
        symbol: [u8; 16],
        sym_len: u8,
        price: f64,
        timestamp: i64,
    },
    ClosePartial {
        symbol: [u8; 16],
        sym_len: u8,
        price: f64,
        quantity: f64,
        timestamp: i64,
    },
    ModifyStopLoss {
        symbol: [u8; 16],
        sym_len: u8,
        price: f64,
        stop_loss: f64,
        timestamp: i64,
    },
    Hold,
}

impl TradingEvent {
    pub fn symbol_str(&self) -> &str {
        match self {
            TradingEvent::OpenPosition {
                symbol, sym_len, ..
            } => std::str::from_utf8(&symbol[..*sym_len as usize]).unwrap_or(""),
            TradingEvent::ClosePosition {
                symbol, sym_len, ..
            } => std::str::from_utf8(&symbol[..*sym_len as usize]).unwrap_or(""),
            TradingEvent::ClosePartial {
                symbol, sym_len, ..
            } => std::str::from_utf8(&symbol[..*sym_len as usize]).unwrap_or(""),
            TradingEvent::ModifyStopLoss {
                symbol, sym_len, ..
            } => std::str::from_utf8(&symbol[..*sym_len as usize]).unwrap_or(""),
            TradingEvent::Hold => "",
        }
    }
}

impl Default for TradingEvent {
    fn default() -> Self {
        TradingEvent::Hold
    }
}
