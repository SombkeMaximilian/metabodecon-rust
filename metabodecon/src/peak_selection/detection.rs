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
}
