use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::infrastructure::disruptor::ring_buffer::RingBuffer;

#[repr(C, align(64))]
pub struct PaddedCursor {
    pub value: AtomicU64,
}

pub struct Producer<T> {
    ring: Arc<RingBuffer<T>>,
    next_seq: u64,
    consumer_cursors: Vec<Arc<PaddedCursor>>,
}

impl PaddedCursor {
    pub fn new(initial: u64) -> Self {
        PaddedCursor {
            value: AtomicU64::new(initial),
        }
    }
}

impl<T: Default + Copy> Producer<T> {
    pub fn new(ring: Arc<RingBuffer<T>>, consumer_cursors: Vec<Arc<PaddedCursor>>) -> Self {
        Producer {
            ring,
            next_seq: 0,
            consumer_cursors,
        }
    }
    pub fn publish(&mut self, value: T) {
        let seq = self.next_seq;
        let wrap_point = (seq as i64).wrapping_sub(self.ring.capacity() as i64 - 1);
        loop {
            let min = self
                .consumer_cursors
                .iter()
                .map(|c| c.value.load(Ordering::Acquire) as i64)
                .min()
                .unwrap_or(i64::MAX);
            if min >= wrap_point {
                break;
            }
            std::hint::spin_loop();
        }
        unsafe {
            self.ring.write(seq, value);
        }
        self.ring.slot_sequence(seq).store(seq, Ordering::Release);
        self.next_seq = seq.wrapping_add(1);
    }
}
