use crate::data::Spectrum;
use crate::smoothing::{SmoothingAlgo};

#[allow(dead_code, unused_variables)]
pub fn preprocess_spectrum(spectrum: &mut Spectrum, smoothing_algo: SmoothingAlgo) {}

#[allow(dead_code, unused_variables)]
fn remove_water_signal(intensities: &mut [f64], water_region_width: f64) {}

#[allow(dead_code, unused_variables)]
fn remove_negative_values(intensities: &mut [f64]) {}

#[allow(dead_code, unused_variables)]
fn apply_smoothing(intensities: &mut [f64], algorithm: SmoothingAlgo) {}
