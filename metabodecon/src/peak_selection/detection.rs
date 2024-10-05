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
