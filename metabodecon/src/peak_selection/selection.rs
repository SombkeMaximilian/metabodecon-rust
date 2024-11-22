use crate::data_structures::Peak;
use crate::data_structures::Spectrum;
use crate::peak_selection::detection::Detector;
use crate::peak_selection::filtering::filter_peaks;

pub fn select_peaks(spectrum: Spectrum, threshold: f64) -> Vec<Peak> {
    let signal_boundaries = spectrum.signal_boundaries_indices();
    let mut second_derivative = second_derivative(spectrum.intensities());
    let peaks = {
        let detector = Detector::new(&second_derivative);
        detector.detect_peaks()
    };
    second_derivative.iter_mut().for_each(|d| *d = d.abs());
    filter_peaks(peaks, &second_derivative, signal_boundaries, threshold)
}

fn second_derivative(intensities: &[f64]) -> Vec<f64> {
    intensities
        .windows(3)
        .map(|w| w[0] - 2. * w[1] + w[2])
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn second_derivative() {
        let intensities = vec![1., 2., 3., 2., 1.];
        let expected = vec![0., -2., 0.];
        assert_eq!(super::second_derivative(&intensities), expected);
    }
}
