use crate::spectrum::Spectrum;
use crate::peak_selection::Peak;

#[derive(Debug, Clone)]
pub struct ReducedSpectrum {
    chemical_shifts: Vec<f64>,
    intensities: Vec<f64>,
}

impl ReducedSpectrum {
    pub fn new(spectrum: &Spectrum, peaks: &[Peak]) -> Self {
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

    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
    }

    pub fn intensities(&self) -> &[f64] {
        &self.intensities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut spectrum = Spectrum::new(
            vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.],
            vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1.],
            (2., 9.),
            (5.45, 5.55),
        );
        spectrum.set_intensities(vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1.]);
        let peaks = vec![Peak::new(2, 3, 4), Peak::new(4, 5, 6), Peak::new(6, 7, 8)];
        let reduced = ReducedSpectrum::new(&spectrum, &peaks);
        assert_eq!(
            reduced.chemical_shifts,
            vec![3., 4., 5., 5., 6., 7., 7., 8., 9.]
        );
        assert_eq!(
            reduced.intensities,
            vec![8., 7., 6., 6., 5., 4., 4., 3., 2.]
        );
    }

    #[test]
    fn accessors() {
        let reduced = ReducedSpectrum {
            chemical_shifts: vec![1., 2., 3.],
            intensities: vec![4., 5., 6.],
        };
        assert_eq!(reduced.chemical_shifts(), vec![1., 2., 3.]);
        assert_eq!(reduced.intensities(), vec![4., 5., 6.]);
    }
}
