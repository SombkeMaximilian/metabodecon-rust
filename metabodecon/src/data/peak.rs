#[derive(Clone)]
pub struct Peak {
    left: usize,
    center: usize,
    right: usize
}

impl Peak {
    pub fn new() -> Self {
        Peak {
            left: 0,
            center: 0,
            right: 0
        }
    }

    pub fn from_pos(left: usize, center: usize, right: usize) -> Self {
        Peak {
            left,
            center,
            right
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

    pub fn set_left(&mut self, left: usize) {
        self.left = left;
    }

    pub fn set_center(&mut self, center: usize) {
        self.center = center;
    }

    pub fn set_right(&mut self, right: usize) {
        self.right = right;
    }
}
