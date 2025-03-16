use crate::alignment::feature::FeatureMap;

pub(crate) trait Solver: Send + Sync + std::fmt::Debug {
    fn solve(&self, candidate_maps: Vec<FeatureMap>) -> Vec<FeatureMap>;
}
