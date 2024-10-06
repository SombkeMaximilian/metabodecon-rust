use crate::data::Peak;

#[allow(dead_code, unused_variables)]
pub fn filter_peaks(peaks: Vec<Peak>, abs_second_derivative: Vec<f64>) {}

#[allow(dead_code, unused_variables)]
fn score_peaks(peaks: Vec<Peak>, abs_second_derivative: Vec<f64>) -> Vec<f64> {
    peaks.iter()
        .map(|peak| {
            f64::min(abs_second_derivative[peak.left()-1..peak.center()].iter().sum(),
                     abs_second_derivative[peak.center()-1..peak.right()].iter().sum())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_peaks() {
        let peaks = vec![Peak::from_pos(1, 3, 4), Peak::from_pos(5, 6, 9)];
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        assert_eq!(score_peaks(peaks, abs_second_derivative), vec![6., 7.]);
    }
}
