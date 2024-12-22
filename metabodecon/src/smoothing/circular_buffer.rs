use num_traits::Zero;

#[derive(Debug)]
pub struct CircularBuffer<Type> {
    buffer: Box<[Type]>,
    index: usize,
    num_elements: usize,
    capacity: usize,
}

impl<Type: Copy + Zero> CircularBuffer<Type> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![Type::zero(); capacity].into_boxed_slice(),
            index: 0,
            num_elements: 0,
            capacity,
        }
    }

    pub fn next(&mut self, value: Type) -> Option<Type> {
        let popped_value: Option<Type> = if self.num_elements == self.capacity {
            self.pop()
        } else {
            None
        };
        self.push(value);
        popped_value
    }

    pub fn push(&mut self, value: Type) {
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % self.capacity;
        if self.num_elements < self.capacity {
            self.num_elements += 1;
        }
    }

    pub fn pop(&mut self) -> Option<Type> {
        if self.num_elements == 0 {
            return None;
        }
        let index: usize = (self.index + self.capacity - self.num_elements) % self.capacity;
        self.num_elements -= 1;
        Some(self.buffer[index])
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.num_elements = 0;
    }

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
