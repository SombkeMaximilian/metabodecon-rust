use crate::data::Peak;

#[allow(dead_code, unused_variables)]
pub fn detect_peaks(intensities: &[f64]) -> Vec<Peak> { Vec::<Peak>::new() }

#[allow(dead_code, unused_variables)]
fn second_derivative(intensities: &[f64]) -> Vec<f64> {
    intensities.windows(3)
        .map(|w| w[0] - 2. * w[1] + w[2])
        .collect()
}

#[allow(dead_code, unused_variables)]
fn find_peak_centers(second_derivative: &[f64]) -> Vec<usize> {
    second_derivative.windows(3)
        .enumerate()
        .filter(|(_, w)| w[1] < w[0] && w[1] < 0. && w[1] < w[2])
        .map(|(i, _)| i + 2)
        .collect()
}

#[allow(dead_code, unused_variables)]
fn find_peak_borders(second_derivative: &[f64], peak_centers: &[usize]) -> Vec<(usize, usize)> {
    peak_centers.iter()
        .map(|&i| {
            (i - find_left_border(&second_derivative[0..i]),
             i + find_right_border(&second_derivative[i-1..]))
        })
        .collect()
}

fn find_right_border(second_derivative_right: &[f64]) -> usize {
    second_derivative_right.windows(3)
        .skip_while(|w| w[1] <= w[0])
        .position(|w| w[1] >= w[2] || (w[1] < 0. && w[2] >= 0.))
        .map_or(usize::MAX, |i| i + 1)
}

fn find_left_border(second_derivative_left: &[f64]) -> usize {
    second_derivative_left.windows(3)
        .rev()
        .skip_while(|w| w[1] <= w[2])
        .position(|w| w[1] >= w[0] || (w[1] < 0. && w[0] >= 0.))
        .map_or(usize::MAX, |i| i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_derivative() {
        let intensities = vec![1., 2., 3., 2., 1.];
        let expected = vec![0., -2., 0.];
        assert_eq!(second_derivative(&intensities), expected);
    }

    #[test]
    fn test_find_peak_centers() {
        let intensities = vec![1., 2., 3., 2., 1.];
        let second_derivative = second_derivative(&intensities);
        let expected = vec![2];
        assert_eq!(find_peak_centers(&second_derivative), expected);
    }

    #[test]
    fn test_find_peak_borders() {
        // indices are offset by 1, as second derivative is computed for central points only
        let mut second_derivative = vec![0.5, -0.5, -1., 0., 0.5, 0.];
        assert_eq!(find_peak_borders(&second_derivative, &[3]), vec![(2, 5)]);
        second_derivative = vec![0., 0.5, 0., -1., -0.5, 0.5];
        assert_eq!(find_peak_borders(&second_derivative, &[4]), vec![(2, 5)])
    }

    #[test]
    fn test_find_right_border() {
        let mut second_derivative = vec![0., -2., -1., -0.5, 0.5];
        assert_eq!(find_right_border(&second_derivative[2..]), 1);
        second_derivative = vec![0., -2., -1., 0., 0.5, 0.];
        assert_eq!(find_right_border(&second_derivative[2..]), 2);
    }

    #[test]
    fn test_find_left_border() {
        let mut second_derivative = vec![0.5, -0.5, -1., -2., 0.];
        assert_eq!(find_left_border(&second_derivative[0..=2]), 1);
        second_derivative = vec![0., 0.5, 0., -1., -2., 0.];
        assert_eq!(find_left_border(&second_derivative[0..=3]), 2);
    }
}
