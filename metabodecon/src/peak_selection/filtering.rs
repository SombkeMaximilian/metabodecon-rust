use crate::data_structures::Peak;
use crate::peak_selection::scorer::*;

pub fn filter_peaks(
    mut peaks: Vec<Peak>,
    abs_second_derivative: &[f64],
    signal_boundaries: (usize, usize),
    threshold: f64,
) -> Vec<Peak> {
    let scorer = ScorerMinimumSum::new(abs_second_derivative);
    let boundaries = peak_region_boundaries(&peaks, signal_boundaries);
    let left_sfr = &peaks[0..boundaries.0];
    let right_sfr = &peaks[boundaries.1..];
    let scores_sfr: Vec<f64> = left_sfr
        .iter()
        .chain(right_sfr.iter())
        .map(|peak| scorer.score_peak(peak))
        .collect();
    let (mean, sd) = mean_sd_sfr_scores(scores_sfr);
    peaks
        .drain(boundaries.0..boundaries.1)
        .filter(|peak| scorer.score_peak(peak) >= mean + threshold * sd)
        .collect()
}

fn peak_region_boundaries(peaks: &[Peak], signal_boundaries: (usize, usize)) -> (usize, usize) {
    let left = peaks
        .iter()
        .position(|peak| peak.center() > signal_boundaries.0)
        .unwrap();
    let right = peaks[left..]
        .iter()
        .position(|peak| peak.center() > signal_boundaries.1)
        .map(|i| left + i)
        .unwrap();
    (left, right)
}

fn mean_sd_sfr_scores(scores_sfr: Vec<f64>) -> (f64, f64) {
    let mean: f64 = scores_sfr.iter().sum::<f64>() / scores_sfr.len() as f64;
    let variance: f64 = scores_sfr
        .iter()
        .map(|score| (score - mean).powi(2))
        .sum::<f64>()
        / scores_sfr.len() as f64;
    (mean, variance.sqrt())
}

#[cfg(test)]
mod tests {
    use crate::data_structures::Peak;
    use crate::peak_selection::scorer::*;

    #[test]
    fn peak_region_boundaries() {
        let signal_region_boundaries: (usize, usize) = (3, 7);
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        assert_eq!(
            super::peak_region_boundaries(&peaks, signal_region_boundaries),
            (1, 3)
        );
    }

    #[test]
    fn mean_sd_sfr_scores() {
        let peaks: Vec<Peak> = vec![2, 4, 5, 8]
            .into_iter()
            .map(|i| Peak::new(i - 1, i, i + 1))
            .collect();
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        let scorer = ScorerMinimumSum::new(&abs_second_derivative);
        let peak_region_boundaries = (1, 3);
        let scores_sfr: Vec<f64> = peaks[0..peak_region_boundaries.0]
            .iter()
            .chain(peaks[peak_region_boundaries.1..].iter())
            .map(|peak| scorer.score_peak(peak))
            .collect();
        let (mean, sd) = super::mean_sd_sfr_scores(scores_sfr);
        assert_eq!(mean, 4.0);
        assert_eq!(sd, 1.0);
    }
}
