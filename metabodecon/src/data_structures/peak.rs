#[derive(Debug, Copy, Clone)]
pub struct Peak {
    left: usize,
    center: usize,
    right: usize,
}

impl Peak {
    pub fn new(left: usize, center: usize, right: usize) -> Self {
        Peak {
            left,
            center,
            right,
        }
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn center(&self) -> usize {
        self.center
    }

    pub fn right(&self) -> usize {
        self.right
    }
}
