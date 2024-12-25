/// Data structure that represents a peak in a spectrum.
#[derive(Debug)]
pub struct Peak {
    /// Index of the left boundary of the peak.
    left: usize,
    /// Index of the center of the peak.
    center: usize,
    /// Index of the right boundary of the peak.
    right: usize,
}

impl Peak {
    /// Creates a `Peak` from the indices of the 3 points that define it.
    pub fn new(left: usize, center: usize, right: usize) -> Self {
        Self {
            left,
            center,
            right,
        }
    }

    /// Returns the index of the left boundary of the peak.
    pub fn left(&self) -> usize {
        self.left
    }

    /// Returns the index of the center of the peak.
    pub fn center(&self) -> usize {
        self.center
    }

    /// Returns the index of the right boundary of the peak.
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
