use crate::data::Peak;

#[allow(dead_code, unused_variables)]
pub fn filter_peaks(peaks: &[Peak], abs_second_derivative: &[f64]) {}

#[allow(dead_code, unused_variables)]
fn score_peak(peak: &Peak, abs_second_derivative: &[f64]) -> f64 {
    f64::min(abs_second_derivative[peak.left()-1..peak.center()].iter().sum(),
             abs_second_derivative[peak.center()-1..peak.right()].iter().sum())
}

#[allow(dead_code, unused_variables)]
fn peak_region_boundaries(peaks: &[Peak], signal_boundaries: (usize, usize)) -> (usize, usize) {
    let left = peaks.iter()
        .position(|peak| peak.center() > signal_boundaries.0)
        .unwrap();
    let right = peaks[left..].iter()
        .position(|peak| peak.center() > signal_boundaries.1)
        .map(|i| left + i)
        .unwrap();
    (left, right)
}

#[allow(dead_code, unused_variables)]
fn mean_sd_sfr_scores(peaks: &[Peak], abs_second_derivative: &[f64],
                                      peak_region_boundaries: (usize, usize)) -> (f64, f64) {
    let left_sfr = &peaks[0..peak_region_boundaries.0];
    let right_sfr = &peaks[peak_region_boundaries.1..];
    let scores_sfr: Vec<f64> = left_sfr.iter()
        .chain(right_sfr.iter())
        .map(|peak| score_peak(peak, &abs_second_derivative))
        .collect();
    let mean : f64 = scores_sfr.iter().sum::<f64>() / (left_sfr.len() + right_sfr.len()) as f64;
    let standard_deviation : f64 = scores_sfr.iter()
        .map(|score| (score - mean).powi(2))
        .sum::<f64>()
        / (left_sfr.len() + right_sfr.len()) as f64;
    (mean, standard_deviation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_peak() {
        let peaks = vec![Peak::from_pos(1, 3, 4), Peak::from_pos(5, 6, 9)];
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        let scores : Vec<f64> = peaks.iter()
            .map(|peak| score_peak(peak, &abs_second_derivative))
            .collect();
        assert_eq!(scores, vec![6., 7.]);
    }
    
    #[test]
    fn test_peak_region_boundaries() {
        let signal_region_boundaries : (usize, usize) = (3, 7);
        let peaks : Vec<Peak> = vec![2, 4, 5, 8].into_iter()
            .map(|i| Peak::from_pos(i-1, i, i+1))
            .collect();
        assert_eq!(peak_region_boundaries(&peaks, signal_region_boundaries), (1, 3));
    }
}
