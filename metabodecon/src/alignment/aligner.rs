use crate::alignment::Alignment;
use crate::alignment::assignment::{
    AssignmentChain, DistanceSimilarityFilter, Filter, FilteringSettings,
};
use crate::alignment::feature::{
    AlignmentStrategy, FeatureLayer, FeatureMap, PairwiseStrategy, ReferenceStrategy, Strategy,
};
use crate::alignment::solving::{LinearProgramming, Solver, SolvingSettings};
use crate::deconvolution::Deconvolution;
use crate::{Result, Settings};
use std::sync::Arc;

/// Alignment pipeline for deconvolutions.
#[derive(Clone, Debug)]
pub struct Aligner {
    strategy: Arc<dyn Strategy>,
    filter: Arc<dyn Filter>,
    solver: Arc<dyn Solver>,
}

impl Default for Aligner {
    fn default() -> Self {
        Self::new(
            AlignmentStrategy::default(),
            FilteringSettings::default(),
            SolvingSettings::default(),
        )
        .unwrap()
    }
}

impl Aligner {
    pub fn new(
        alignment_strategy: AlignmentStrategy,
        filtering_settings: FilteringSettings,
        solving_settings: SolvingSettings,
    ) -> Result<Self> {
        alignment_strategy.validate()?;
        filtering_settings.validate()?;
        solving_settings.validate()?;

        let strategy: Arc<dyn Strategy> = match alignment_strategy {
            AlignmentStrategy::Pairwise => Arc::new(PairwiseStrategy::new()),
            AlignmentStrategy::Reference(index) => Arc::new(ReferenceStrategy::new(index)),
        };
        let filter: Arc<dyn Filter> = match filtering_settings {
            FilteringSettings::DistanceSimilarity {
                similarity_metric,
                max_distance,
                min_similarity,
            } => Arc::new(DistanceSimilarityFilter::new(
                similarity_metric,
                max_distance,
                min_similarity,
            )),
        };
        let solver: Arc<dyn Solver> = match solving_settings {
            SolvingSettings::LinearProgramming => Arc::new(LinearProgramming),
        };

        Ok(Self {
            strategy,
            filter,
            solver,
        })
    }

    pub fn alignment_strategy(&self) -> AlignmentStrategy {
        self.strategy.settings()
    }

    pub fn filtering_settings(&self) -> FilteringSettings {
        self.filter.settings()
    }

    pub fn solving_settings(&self) -> SolvingSettings {
        self.solver.settings()
    }

    pub fn set_alignment_strategy(&mut self, strategy: AlignmentStrategy) -> Result<()> {
        strategy.validate()?;
        self.strategy = match strategy {
            AlignmentStrategy::Pairwise => Arc::new(PairwiseStrategy::new()),
            AlignmentStrategy::Reference(index) => Arc::new(ReferenceStrategy::new(index)),
        };

        Ok(())
    }

    pub fn set_filtering_settings(&mut self, settings: FilteringSettings) -> Result<()> {
        settings.validate()?;
        self.filter = match settings {
            FilteringSettings::DistanceSimilarity {
                similarity_metric,
                max_distance,
                min_similarity,
            } => Arc::new(DistanceSimilarityFilter::new(
                similarity_metric,
                max_distance,
                min_similarity,
            )),
        };

        Ok(())
    }

    pub fn set_solving_settings(&mut self, settings: SolvingSettings) -> Result<()> {
        settings.validate()?;
        self.solver = match settings {
            SolvingSettings::LinearProgramming => Arc::new(LinearProgramming),
        };

        Ok(())
    }

    pub fn align_deconvolutions<D: AsRef<Deconvolution>>(&self, deconvolutions: &[D]) -> Alignment {
        let layer_count = deconvolutions.len();
        let mut feature_layers = deconvolutions
            .iter()
            .map(FeatureLayer::from)
            .collect::<Vec<_>>();
        let feature_maps = self
            .strategy
            .generate_maps(&feature_layers, self.filter.as_ref());
        let solution_maps = self.solver.solve(feature_maps);
        let chains = Self::make_chains(solution_maps, layer_count);
        chains.iter().for_each(|chain| {
            let first = chain.iter().next().unwrap();
            let maxp = feature_layers[*first.0].as_ref()[*first.1].maxp();
            chain.iter().skip(1).for_each(|(layer, feature)| {
                feature_layers[*layer].as_mut()[*feature].set_maxp(maxp);
            })
        });
        let deconvolutions = feature_layers
            .into_iter()
            .zip(deconvolutions.iter())
            .map(|(feature_layer, deconvolution)| {
                let deconvolution = deconvolution.as_ref();

                Deconvolution::new(
                    feature_layer
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    deconvolution.smoothing_settings(),
                    deconvolution.selection_settings(),
                    deconvolution.fitting_settings(),
                    f64::NAN,
                )
            });

        Alignment::new(deconvolutions)
    }

    fn make_chains(solution_maps: Vec<FeatureMap>, layer_count: usize) -> Vec<AssignmentChain> {
        let mut chains = Vec::<AssignmentChain>::new();
        solution_maps
            .into_iter()
            .for_each(|solution_map| {
                let i = solution_map.layer_i();
                let j = solution_map.layer_j();
                solution_map
                    .assignments()
                    .iter()
                    .for_each(|assignment| {
                        let a = assignment.feature_a();
                        let b = assignment.feature_b();
                        let position = chains.iter().position(|chain| {
                            chain.iter().any(|(layer, feature)| {
                                *layer == i && *feature == a || *layer == j && *feature == b
                            })
                        });
                        if let Some(position) = position {
                            chains[position].push(i, a);
                            chains[position].push(j, b);
                        } else {
                            let mut chain = AssignmentChain::new(layer_count);
                            chain.push(i, a);
                            chain.push(j, b);
                            chains.push(chain);
                        }
                    });
            });
        chains
            .iter_mut()
            .for_each(|chain| chain.drop_duplicates());

        chains
    }
}
