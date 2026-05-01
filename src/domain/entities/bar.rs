#[derive(Debug, Clone, Copy, Default)]
pub struct Bar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub timestamp: i64,
}
#[derive(Debug, Clone, Copy)]
pub enum TimeFrame {
    Min15,
    H1,
    H4,
}
#[derive(Debug, Clone)]
pub struct BarBuffer<const N: usize> {
    slots: [Bar; N],
    head: usize,
    len: usize,
}

impl TimeFrame {
    pub fn duration_nanos(&self) -> i64 {
        match self {
            TimeFrame::Min15 => 15 * 60 * 1_000_000_000,
            TimeFrame::H1 => 60 * 60 * 1_000_000_000,
            TimeFrame::H4 => 4 * 60 * 60 * 1_000_000_000,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            TimeFrame::Min15 => 0,
            TimeFrame::H1 => 1,
            TimeFrame::H4 => 2,
        }
    }
}

impl Bar {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: u64, timestamp: i64) -> Self {
        Bar {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        }
    }

    pub fn update(&mut self, price: f64) {
        if price > self.high {
            self.high = price;
        }
        if price < self.low {
            self.low = price
        }
        self.close = price;
        self.volume += 1;
    }
}

impl<const N: usize> BarBuffer<N> {
    pub fn new() -> Self {
        BarBuffer {
            slots: [Bar::default(); N],
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, bar: Bar) {
        self.slots[self.head] = bar;
        self.head = (self.head + 1) % N;
        if self.len < N {
            self.len += 1;
        }
    }
    fn get(&self, offset: usize) -> Option<&Bar> {
        if offset >= self.len {
            return None;
        }
        let index = (self.head + N - 1 - offset) % N;
        Some(&self.slots[index])
    }
}
