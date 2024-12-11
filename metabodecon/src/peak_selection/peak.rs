#[derive(Debug)]
pub struct Peak {
    left: usize,
    center: usize,
    right: usize,
}

impl Peak {
    pub fn new(left: usize, center: usize, right: usize) -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let peak = Peak::new(1, 2, 3);
        assert_eq!(peak.left(), 1);
        assert_eq!(peak.center(), 2);
        assert_eq!(peak.right(), 3);
    }
}
