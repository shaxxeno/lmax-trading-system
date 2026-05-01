use std::{cell::UnsafeCell, sync::atomic::AtomicU64};

pub struct Slot<T> {
    sequence: AtomicU64,
    value: UnsafeCell<T>,
}

// SAFETY: sequence protocol guarantees exclusive access per slot
unsafe impl<T: Send> Sync for Slot<T> {}
unsafe impl<T: Send> Send for RingBuffer<T> {}
unsafe impl<T: Send> Sync for RingBuffer<T> {}

pub struct RingBuffer<T> {
    slots: Box<[Slot<T>]>,
    capacity: usize,
    mask: usize,
}

impl<T: Default + Copy> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let slots: Box<[Slot<T>]> = (0..capacity)
            .map(|i| Slot {
                sequence: AtomicU64::new((i as u64).wrapping_sub(capacity as u64)),
                value: UnsafeCell::new(T::default()),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mask = capacity - 1;
        RingBuffer {
            slots,
            capacity,
            mask,
        }
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn slot_sequence(&self, seq: u64) -> &AtomicU64 {
        let index = (seq as usize) & self.mask;
        &self.slots[index].sequence
    }

    pub unsafe fn write(&self, seq: u64, value: T) {
        unsafe {
            let index = (seq as usize) & self.mask;
            *self.slots[index].value.get() = value;
        }
    }

    pub unsafe fn read(&self, seq: u64) -> T {
        unsafe {
            let index = (seq as usize) & self.mask;
            *self.slots[index].value.get()
        }
    }
}
