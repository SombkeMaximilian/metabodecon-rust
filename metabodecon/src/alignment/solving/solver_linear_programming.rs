use crate::alignment::feature::FeatureMap;
use crate::alignment::solving::{AlignmentProblem, Solver};

#[derive(Debug)]
pub(crate) struct LinearProgramming;

impl Solver for LinearProgramming {
    fn solve(&self, candidate_maps: Vec<FeatureMap>) -> Vec<FeatureMap> {
        let mut problem = AlignmentProblem::default();
        problem.add_assignments(&candidate_maps);

        problem.best_assignment()
    }
}
