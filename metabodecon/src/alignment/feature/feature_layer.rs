use crate::alignment::feature::{Assignment, FeaturePoint};
use crate::deconvolution::Deconvolution;

#[derive(Debug)]
pub(crate) struct FeatureLayer {
    features: Vec<FeaturePoint>,
}

impl AsRef<[FeaturePoint]> for FeatureLayer {
    fn as_ref(&self) -> &[FeaturePoint] {
        &self.features
    }
}

impl AsMut<[FeaturePoint]> for FeatureLayer {
    fn as_mut(&mut self) -> &mut [FeaturePoint] {
        &mut self.features
    }
}

impl<T: AsRef<Deconvolution>> From<T> for FeatureLayer {
    fn from(value: T) -> Self {
        value.as_ref().lorentzians().iter().collect()
    }
}

impl<A: Into<FeaturePoint>> FromIterator<A> for FeatureLayer {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let features = iter.into_iter().map(Into::into).collect();

        Self { features }
    }
}

impl IntoIterator for FeatureLayer {
    type Item = FeaturePoint;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

impl FeatureLayer {
    pub(crate) fn assignment_candidates(
        &self,
        other: &Self,
        max_distance: f64,
        min_similarity: f64,
    ) -> Vec<Assignment> {
        self.features
            .iter()
            .enumerate()
            .flat_map(|(i, feature)| {
                other
                    .features
                    .iter()
                    .enumerate()
                    .filter_map(|(j, other_feature)| {
                        let distance = feature.distance(other_feature);
                        let similarity = feature.similarity(other_feature);

                        if distance <= max_distance && similarity >= min_similarity {
                            Some(Assignment::new(i, j, feature.similarity(other_feature)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
