use num_traits::Zero;

/// FIFO buffer with a fixed capacity that wraps around and overwrites old
/// elements when full.
#[derive(Debug)]
pub struct CircularBuffer<Type> {
    /// The underlying storage for the buffer.
    buffer: Box<[Type]>,
    /// The index of the next element to be pushed.
    index: usize,
    /// The number of elements currently in the buffer.
    num_elements: usize,
}

impl<Type: Copy + Zero> CircularBuffer<Type> {
    /// Creates a new `CircularBuffer` with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        Self {
            buffer: vec![Type::zero(); capacity].into_boxed_slice(),
            index: 0,
            num_elements: 0,
        }
    }

    /// Inserts a new element into the buffer and returns the oldest element if
    /// the buffer was already full or `None` otherwise.
    pub fn next(&mut self, value: Type) -> Option<Type> {
        let popped_value: Option<Type> = if self.num_elements == self.buffer.len() {
            self.pop()
        } else {
            None
        };
        self.push(value);
        popped_value
    }

    /// Inserts a new element into the buffer.
    pub fn push(&mut self, value: Type) {
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % self.buffer.len();
        if self.num_elements < self.buffer.len() {
            self.num_elements += 1;
        }
    }

    /// Removes and returns the oldest element from the buffer or `None` if the
    /// buffer was already empty.
    pub fn pop(&mut self) -> Option<Type> {
        if self.num_elements == 0 {
            return None;
        }
        let index: usize = (self.index + self.buffer.len() - self.num_elements) % self.buffer.len();
        self.num_elements -= 1;
        Some(self.buffer[index])
    }

    /// Resets the buffer to its initial state.
    pub fn clear(&mut self) {
        self.index = 0;
        self.num_elements = 0;
    }

    /// Returns the number of elements currently in the buffer.
    pub fn num_elements(&self) -> usize {
        self.num_elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        assert_eq!(buffer.num_elements(), 0);
    }

    #[test]
    fn push() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        assert_eq!(buffer.num_elements(), 1);
    }

    #[test]
    fn pop() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.num_elements(), 1);
    }

    #[test]
    fn next() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.next(4), Some(1));
        assert_eq!(buffer.num_elements(), 3);
    }

    #[test]
    fn clear() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.clear();
        assert_eq!(buffer.num_elements(), 0);
    }
}
