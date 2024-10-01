use crate::data::Spectrum;
use crate::smoothing::{SmoothingAlgo};

pub fn preprocess_spectrum(spectrum: &mut Spectrum, smoothing_algo: SmoothingAlgo) {
    let water_region_width = spectrum.water_region_width();
    remove_water_signal(spectrum.intensities_mut(), water_region_width);
    remove_negative_values(spectrum.intensities_mut());
    smooth_intensities(spectrum.intensities_mut(), smoothing_algo);
}

#[allow(dead_code, unused_variables)]
fn remove_water_signal(intensities: &mut [f64], water_region_width: f64) {}

#[allow(dead_code, unused_variables)]
fn remove_negative_values(intensities: &mut [f64]) {}

#[allow(dead_code, unused_variables)]
fn apply_smoothing(intensities: &mut [f64], algorithm: SmoothingAlgo) {}
