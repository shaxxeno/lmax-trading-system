pub struct LatencyStats {
    min:   u64,
    max:   u64,
    sum:   u64,
    count: u64,
    p99_buf: [u64; 10_000],
}

impl LatencyStats {
    pub fn new() -> Self {
        LatencyStats {
            min:     u64::MAX,
            max:     0,
            sum:     0,
            count:   0,
            p99_buf: [0u64; 10_000],
        }
    }

    pub fn record(&mut self, nanos: u64) {
        if nanos < self.min { self.min = nanos; }
        if nanos > self.max { self.max = nanos; }
        self.sum += nanos;
        self.p99_buf[(self.count % 10_000) as usize] = nanos;
        self.count += 1;

        if self.count % 10_000 == 0 {
            self.print();
        }
    }

    fn p99(&mut self) -> u64 {
        let mut sorted = self.p99_buf;
        sorted.sort_unstable();
        sorted[9_900]
    }

    fn print(&mut self) {
        let avg = self.sum / self.count;
        let p99 = self.p99();
        tracing::info!(
            "Latency — min: {}ns  avg: {}ns  p99: {}ns  max: {}ns  ticks: {}",
            self.min, avg, p99, self.max, self.count
        );
    }
}
