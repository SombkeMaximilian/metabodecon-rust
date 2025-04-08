use crate::alignment::feature::FeatureMap;

/// Trait interface for solving the assignment problem.
pub(crate) trait Solver: Send + Sync + std::fmt::Debug {
    /// Solves the assignment problem for the given feature maps.
    fn solve(&self, candidate_maps: Vec<FeatureMap>) -> Vec<FeatureMap>;
}

/// Solving settings for the assignment problem.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub enum SolvingSettings {
    /// Formulates the assignment problem in a linear programming framework.
    ///
    /// Uses the [good_lp] crate with the [HiGHS] solver to solve the assignment
    /// problem.
    ///
    /// [good_lp]: https://docs.rs/good_lp/
    /// [HiGHS]: https://highs.dev/
    #[default]
    LinearProgramming,
}
