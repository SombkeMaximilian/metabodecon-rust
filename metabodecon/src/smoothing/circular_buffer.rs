pub(super) struct CircularBuffer<Type, const N: usize> {
    buffer: [Type; N],
    index: usize,
    num_elements: usize,
}

impl<Type: Copy, const N: usize> CircularBuffer<Type, N> {
    pub fn new(value: Type) -> Self {
        Self {
            buffer: [value; N],
            index: 0,
            num_elements: 0
        }
    }

    pub fn next(&mut self, value: Type) -> Option<Type> {
        let popped_value : Option<Type> = if self.num_elements == N {
            self.pop()
        } else {
            None
        };
        self.push(value);
        popped_value
    }

    pub fn push(&mut self, value: Type) {
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % N;
        if self.num_elements < N {
            self.num_elements += 1;
        }
    }

    pub fn pop(&mut self) -> Option<Type> {
        if self.num_elements == 0 {
            return None;
        }
        let index : usize = (self.index + N - self.num_elements) % N;
        self.num_elements -= 1;
        Some(self.buffer[index])
    }

    pub fn first(&self) -> Option<Type> {
        if self.num_elements == 0 {
            return None;
        }
        let index : usize = (self.index + N - self.num_elements) % N;
        Some(self.buffer[index])
    }
}
