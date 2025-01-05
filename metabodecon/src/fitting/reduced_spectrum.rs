use crate::peak_selection::Peak;
use crate::spectrum::Spectrum;

/// Data structure that contains a subset of the data from a spectrum.
#[derive(Debug)]
pub struct ReducedSpectrum {
    /// The chemical shifts of the reduced spectrum in ppm.
    chemical_shifts: Vec<f64>,
    /// The intensities of the reduced spectrum in arbitrary units.
    intensities: Vec<f64>,
}

impl ReducedSpectrum {
    /// Extracts the chemical shifts and intensities of the peaks from the
    /// spectrum and constructs a `ReducedSpectrum` from them.
    pub fn new(spectrum: &Spectrum, peaks: &[Peak]) -> Self {
        let chemical_shifts = peaks
            .iter()
            .flat_map(|peak| {
                vec![
                    spectrum.chemical_shifts()[peak.left()],
                    spectrum.chemical_shifts()[peak.center()],
                    spectrum.chemical_shifts()[peak.right()],
                ]
            })
            .collect();
        let intensities = peaks
            .iter()
            .flat_map(|peak| {
                vec![
                    spectrum.intensities()[peak.left()],
                    spectrum.intensities()[peak.center()],
                    spectrum.intensities()[peak.right()],
                ]
            })
            .collect();

        Self {
            chemical_shifts,
            intensities,
        }
    }

    /// Returns the chemical shifts as a slice.
    pub fn chemical_shifts(&self) -> &[f64] {
        &self.chemical_shifts
    }

    /// Returns the intensities as a slice.
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
        )
        .unwrap();
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
