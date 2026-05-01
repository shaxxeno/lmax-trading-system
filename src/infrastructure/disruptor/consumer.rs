use std::sync::{Arc, atomic::Ordering};

use crate::infrastructure::disruptor::{producer::PaddedCursor, ring_buffer::RingBuffer};

pub struct Consumer<T> {
    ring_buffer: Arc<RingBuffer<T>>,
    cursor: Arc<PaddedCursor>,
    next: u64,
}

impl<T: Default + Copy> Consumer<T> {
    pub fn new(ring_buffer: Arc<RingBuffer<T>>, cursor: Arc<PaddedCursor>) -> Self {
        Consumer {
            ring_buffer,
            cursor,
            next: 0,
        }
    }

    pub fn consume(&mut self) -> T {
        loop {
            let seq = self
                .ring_buffer
                .slot_sequence(self.next)
                .load(Ordering::Acquire);
            if seq == self.next {
                break;
            }
            std::hint::spin_loop();
        }
        let value = unsafe { self.ring_buffer.read(self.next) };
        self.cursor.value.store(self.next, Ordering::Release);
        self.next = self.next.wrapping_add(1);
        value
    }
}
