use crate::data::Spectrum;
use crate::smoothing::{Smoother, SmoothingAlgo, MovingAverageSmoother};

pub fn preprocess_spectrum(spectrum: &mut Spectrum, smoothing_algo: SmoothingAlgo) {
    let water_boundaries_indices = spectrum.water_boundaries_indices();
    remove_water_signal(spectrum.intensities_mut(), water_boundaries_indices);
    remove_negative_values(spectrum.intensities_mut());
    smooth_intensities(spectrum.intensities_mut(), smoothing_algo);
}

fn remove_water_signal(intensities: &mut [f64], boundary_indices: (usize, usize)) {
    let min_intensity = *intensities.iter()
                                .min_by(|a, b| a.total_cmp(b))
                                .unwrap_or(&0.);
    let water_region = &mut intensities[boundary_indices.0..boundary_indices.1];
    water_region.fill(min_intensity);
}

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
