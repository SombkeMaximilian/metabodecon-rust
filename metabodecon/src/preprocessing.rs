use crate::data::Spectrum;
use crate::smoothing::{Smoother, SmoothingAlgo, MovingAverageSmoother};

pub fn preprocess_spectrum(spectrum: &mut Spectrum, smoothing_algo: SmoothingAlgo) {
    let water_region_width = spectrum.water_region_width();
    remove_water_signal(spectrum.intensities_mut(), water_region_width);
    remove_negative_values(spectrum.intensities_mut());
    smooth_intensities(spectrum.intensities_mut(), smoothing_algo);
}

#[allow(dead_code, unused_variables)]
fn remove_water_signal(intensities: &mut [f64], water_region_width: f64) {}

fn remove_negative_values(intensities: &mut [f64]) {
    for intensity in intensities.iter_mut() {
        if *intensity < 0.0 {
            *intensity = -*intensity;
        }
    }
}

fn smooth_intensities(intensities: &mut [f64], algorithm: SmoothingAlgo) {
    match algorithm {
        SmoothingAlgo::MovingAverage { algo, iterations, window_size } => {
            let mut smoother = MovingAverageSmoother::<f64>::new(algo, window_size);
            for _ in 0..iterations {
                smoother.smooth_values(intensities);
            }
        }
    }
}
