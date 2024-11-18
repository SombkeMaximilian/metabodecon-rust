use crate::data_structures::Peak;

pub fn detect_peaks(second_derivative: &[f64]) -> Vec<Peak> {
    let peak_centers = find_peak_centers(&second_derivative);
    let peak_borders = find_peak_borders(&second_derivative, &peak_centers);
    peak_centers
        .into_iter()
        .zip(peak_borders.into_iter())
        .filter(|(_, (left, right))| *left != 0 && *right != second_derivative.len() + 1)
        .map(|(center, (left, right))| Peak::new(left, center, right))
        .collect()
}

fn find_peak_centers(second_derivative: &[f64]) -> Vec<usize> {
    second_derivative
        .windows(3)
        .enumerate()
        .filter(|(_, w)| w[1] < w[0] && w[1] < 0. && w[1] < w[2])
        .map(|(i, _)| i + 2)
        .collect()
}

fn find_peak_borders(second_derivative: &[f64], peak_centers: &[usize]) -> Vec<(usize, usize)> {
    peak_centers
        .iter()
        .map(|&i| {
            (
                i - find_left_border(&second_derivative[0..i]),
                i + find_right_border(&second_derivative[i - 1..]),
            )
        })
        .collect()
}

fn find_right_border(second_derivative_right: &[f64]) -> usize {
    second_derivative_right
        .windows(3)
        .position(|w| w[1] > w[0] && (w[1] >= w[2] || (w[1] < 0. && w[2] >= 0.)))
        .map_or(second_derivative_right.len(), |i| i + 1)
}

fn find_left_border(second_derivative_left: &[f64]) -> usize {
    second_derivative_left
        .windows(3)
        .rev()
        .position(|w| w[1] > w[2] && (w[1] >= w[0] || (w[1] < 0. && w[0] >= 0.)))
        .map_or(second_derivative_left.len(), |i| i + 1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_peak_centers() {
        let second_derivative = vec![0., -2., 0.];
        assert_eq!(super::find_peak_centers(&second_derivative), vec![2]);
    }

    #[test]
    fn find_peak_borders() {
        // indices are offset by 1, as second derivative is computed for central points only
        let mut second_derivative = vec![0.5, -0.5, -1., 0., 0.5, 0.];
        assert_eq!(
            super::find_peak_borders(&second_derivative, &[3]),
            vec![(2, 5)]
        );
        second_derivative = vec![0., 0.5, 0., -1., -0.5, 0.5];
        assert_eq!(
            super::find_peak_borders(&second_derivative, &[4]),
            vec![(2, 5)]
        );
        second_derivative = vec![1., 1., 1., 1.5, 1.];
        assert_eq!(
            super::find_peak_borders(&second_derivative, &[3]),
            vec![(0, 4)]
        );
        second_derivative = vec![1., 1.5, 1., 1., 1.];
        assert_eq!(
            super::find_peak_borders(&second_derivative, &[3]),
            vec![(2, 6)]
        );
        second_derivative = vec![1., 1., 1., 1., 1.];
        assert_eq!(
            super::find_peak_borders(&second_derivative, &[3]),
            vec![(0, 6)]
        );
    }

    #[test]
    fn find_right_border() {
        let mut second_derivative = vec![0., -2., -1., -0.5, 0.5];
        assert_eq!(super::find_right_border(&second_derivative[2..]), 1);
        second_derivative = vec![0., -2., -1., 0., 0.5, 0.];
        assert_eq!(super::find_right_border(&second_derivative[2..]), 2);
        second_derivative = vec![1., 1., 1., 1., 1.];
        assert_eq!(super::find_right_border(&second_derivative[2..]), 3);
    }

    #[test]
    fn find_left_border() {
        let mut second_derivative = vec![0.5, -0.5, -1., -2., 0.];
        assert_eq!(super::find_left_border(&second_derivative[0..=2]), 1);
        second_derivative = vec![0., 0.5, 0., -1., -2., 0.];
        assert_eq!(super::find_left_border(&second_derivative[0..=3]), 2);
        second_derivative = vec![1., 1., 1., 1., 1.];
        assert_eq!(super::find_left_border(&second_derivative[0..=2]), 3);
    }
}
