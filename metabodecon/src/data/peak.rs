pub struct Peak {
    left: usize,
    center: usize,
    right: usize,
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
}
