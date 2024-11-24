use crate::data_structures::Spectrum;
use crate::smoothing::{MovingAverageSmoother, Smoother, SmoothingAlgo};

pub fn preprocess_spectrum(spectrum: &mut Spectrum, smoothing_algo: SmoothingAlgo) {
    let water_boundaries_indices = spectrum.water_boundaries_indices();
    let mut intensities = spectrum.intensities_raw().to_vec();
    remove_water_signal(&mut intensities, water_boundaries_indices);
    remove_negative_values(&mut intensities);
    smooth_intensities(&mut intensities, smoothing_algo);
    spectrum.set_intensities(intensities);
}

fn remove_water_signal(intensities: &mut [f64], boundary_indices: (usize, usize)) {
    let min_intensity = *intensities
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(&0.);
    let water_region = &mut intensities[boundary_indices.0..boundary_indices.1];
    water_region.fill(min_intensity);
}

fn remove_negative_values(intensities: &mut [f64]) {
    intensities
        .iter_mut()
        .filter(|intensity| **intensity < 0.0)
        .for_each(|intensity| *intensity = -*intensity);
}

fn smooth_intensities(intensities: &mut [f64], algorithm: SmoothingAlgo) {
    match algorithm {
        SmoothingAlgo::MovingAverage {
            algo,
            iterations,
            window_size,
        } => {
            let mut smoother = MovingAverageSmoother::<f64>::new(algo, iterations, window_size);
            smoother.smooth_values(intensities);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::smoothing::{MovingAverageAlgo, SmoothingAlgo};

    #[test]
    fn remove_water_signal() {
        let mut intensities = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let water_boundaries_indices = (1, 4);
        super::remove_water_signal(&mut intensities, water_boundaries_indices);
        assert_eq!(intensities, vec![1.0, 1.0, 1.0, 1.0, 5.0]);
    }

    #[test]
    fn remove_negative_values() {
        let mut intensities = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        super::remove_negative_values(&mut intensities);
        assert_eq!(intensities, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn smooth_intensities() {
        let mut intensities = vec![1.25, 1.75, 1.5, 2.0, 1.75];
        let algorithm = SmoothingAlgo::MovingAverage {
            algo: MovingAverageAlgo::Simple,
            iterations: 1,
            window_size: 3,
        };
        super::smooth_intensities(&mut intensities, algorithm);
        assert_eq!(intensities, vec![1.5, 1.5, 1.75, 1.75, 1.875]);
    }
}
