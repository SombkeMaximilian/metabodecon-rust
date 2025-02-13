use num_traits::Zero;
use std::collections::VecDeque;

/// FIFO buffer with a fixed capacity that wraps around and overwrites old
/// elements when full.
#[derive(Debug)]
pub(crate) struct CircularBuffer<Type> {
    /// The underlying storage for the buffer.
    buffer: VecDeque<Type>,
    /// The maximum number of elements the buffer can hold.
    ///
    /// This has to be stored separately because [`VecDeque`] only guarantees
    /// that it can hold at least this many elements.
    capacity: usize,
}

impl<Type: Copy + Zero> CircularBuffer<Type> {
    /// Creates a new `CircularBuffer` with the given capacity.
    ///
    /// # Panics
    ///
    /// Panics if the capacity is zero.
    pub(crate) fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");

        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Inserts a new element into the buffer and returns the oldest element if
    /// the buffer was already full or `None` otherwise.
    pub(crate) fn next(&mut self, value: Type) -> Option<Type> {
        let popped_value = if self.buffer.len() == self.capacity {
            self.pop()
        } else {
            None
        };
        self.push(value);

        popped_value
    }

    /// Inserts a new element into the buffer.
    pub(crate) fn push(&mut self, value: Type) {
        self.buffer.push_back(value);
    }

    /// Removes and returns the oldest element from the buffer or `None` if the
    /// buffer was already empty.
    pub(crate) fn pop(&mut self) -> Option<Type> {
        self.buffer.pop_front()
    }

    /// Resets the buffer to its initial state.
    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Returns the number of elements currently in the buffer.
    pub(crate) fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the capacity of the buffer.
    #[cfg(test)]
    pub(crate) fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(10);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 10);
    }

    #[test]
    fn push() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn pop() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn next() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.next(4), Some(1));
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn clear() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.clear();
        assert_eq!(buffer.len(), 0);
    }
}
