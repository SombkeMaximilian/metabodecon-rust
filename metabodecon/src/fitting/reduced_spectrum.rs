use crate::data::Peak;
use crate::data::Spectrum;

pub struct ReducedSpectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
}

impl ReducedSpectrum {
    pub fn new() -> Self {
        Self {
            chemical_shifts: Vec::new(),
            intensities: Vec::new(),
        }
    }

    pub fn from_data(chemical_shifts: Vec<f64>, intensities: Vec<f64>) -> Self {
        Self {
            chemical_shifts,
            intensities,
        }
    }

    pub fn from_spectrum(spectrum: &Spectrum, peaks: &[Peak]) -> Self {
        let positions = peaks
            .iter()
            .flat_map(|peak| vec![peak.left(), peak.center(), peak.right()])
            .collect::<Vec<_>>();
        let chemical_shifts = positions
            .iter()
            .map(|&pos| spectrum.chemical_shifts()[pos])
            .collect();
        let intensities = positions
            .into_iter()
            .map(|pos| spectrum.intensities()[pos])
            .collect();

        Self {
            chemical_shifts,
            intensities,
        }
    }

    pub fn chemical_shifts(&self) -> &Vec<f64> {
        &self.chemical_shifts
    }

    pub fn intensities(&self) -> &Vec<f64> {
        &self.intensities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_spectrum_peaks() {
        let spectrum = Spectrum::from_data(
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.],
            vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1.],
            (2., 9.),
            0.,
        );
        let peaks = vec![
            Peak::from_pos(2, 3, 4),
            Peak::from_pos(4, 5, 6),
            Peak::from_pos(6, 7, 8),
        ];
        let reduced = ReducedSpectrum::from_spectrum(&spectrum, &peaks);
        assert_eq!(reduced.chemical_shifts, vec![3., 4., 5., 5., 6., 7., 7., 8., 9.]);
        assert_eq!(reduced.intensities, vec![8., 7., 6., 6., 5., 4., 4., 3., 2.]);
    }
}
