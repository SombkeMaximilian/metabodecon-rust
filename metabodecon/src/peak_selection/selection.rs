#[allow(dead_code, unused_variables)]
pub fn select_peaks() {

}

#[allow(dead_code, unused_variables)]
fn second_derivative(intensities: &[f64]) -> Vec<f64> {
    intensities.windows(3)
        .map(|w| w[0] - 2. * w[1] + w[2])
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
}
