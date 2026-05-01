use std::sync::Arc;

use crate::infrastructure::disruptor::{
    consumer::Consumer,
    producer::{PaddedCursor, Producer},
    ring_buffer::RingBuffer,
};

pub mod consumer;
pub mod producer;
pub mod ring_buffer;

pub fn build<T: Default + Copy>(
    capacity: usize,
    num_consumers: usize,
) -> (Producer<T>, Vec<Consumer<T>>) {
    let ring_buff = Arc::new(RingBuffer::new(capacity));
    let cursors: Vec<Arc<PaddedCursor>> = (0..num_consumers)
        .map(|_| Arc::new(PaddedCursor::new(u64::MAX)))
        .collect();
    let consumers: Vec<Consumer<T>> = cursors
        .iter()
        .map(|c| Consumer::new(Arc::clone(&ring_buff), Arc::clone(c)))
        .collect();
    let producer = Producer::new(Arc::clone(&ring_buff), cursors);
    (producer, consumers)
}
