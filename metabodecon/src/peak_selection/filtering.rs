use crate::data::Peak;

#[allow(dead_code, unused_variables)]
pub fn filter_peaks(peaks: Vec<Peak>, abs_second_derivative: Vec<f64>) {}

#[allow(dead_code, unused_variables)]
fn score_peaks(peaks: &mut Vec<Peak>, abs_second_derivative: Vec<f64>) {
    peaks.iter_mut()
        .for_each(|peak| {
            let (left, right) : (f64, f64) = (
                abs_second_derivative[peak.left()-1..peak.center()].iter().sum(),
                abs_second_derivative[peak.center()-1..peak.right()].iter().sum()
            );
            peak.set_score(f64::min(left, right))
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_peaks() {
        let mut peaks = vec![Peak::from_pos(1, 3, 4), Peak::from_pos(5, 6, 9)];
        let abs_second_derivative = vec![1., 2., 4., 2., 2., 5., 4., 3., 2.];
        let expected = vec![6., 7.];
        score_peaks(&mut peaks, abs_second_derivative);
        peaks.iter()
            .zip(expected)
            .for_each(|(peak, e)| assert_eq!(peak.score(), e));
    }
}
