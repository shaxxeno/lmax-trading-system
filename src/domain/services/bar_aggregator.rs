use crate::domain::entities::bar::{Bar, BarBuffer, TimeFrame};

const TFS: usize = 3;
const TIMEFRAMES: [TimeFrame; TFS] = [TimeFrame::Min15, TimeFrame::H1, TimeFrame::H4];

struct BarAggregator<const N: usize> {
    bars: [BarBuffer<N>; TFS],
    current_bar: [Option<Bar>; TFS],
    start_time: [i64; TFS],
}

impl<const N: usize> BarAggregator<N> {
    pub fn new() -> Self {
        BarAggregator {
            bars: [BarBuffer::new(), BarBuffer::new(), BarBuffer::new()],
            current_bar: [None, None, None],
            start_time: [0, 0, 0],
        }
    }
    pub fn update(&mut self, price: f64, timestamp: i64) -> [bool; TFS] {
        let mut closed = [false; TFS];

        for i in 0..TFS {
            let duration = TIMEFRAMES[i].duration_nanos();

            match self.current_bar[i] {
                None => {
                    self.current_bar[i] = Some(Bar::new(price, price, price, price, 1, timestamp));
                    self.start_time[i] = timestamp;
                }
                Some(ref mut bar) => {
                    if timestamp >= self.start_time[i] + duration {
                        self.bars[i].push(*bar);
                        self.current_bar[i] =
                            Some(Bar::new(price, price, price, price, 1, timestamp));
                        self.start_time[i] = timestamp;
                        closed[i] = true;
                    } else {
                        bar.update(price);
                    }
                }
            }
        }
        closed
    }
}
