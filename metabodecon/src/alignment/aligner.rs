use crate::alignment::Alignment;
use crate::alignment::assignment::{
    AssignmentChain, DistanceSimilarityFilter, Filter, FilteringSettings,
};
use crate::alignment::feature::{FeatureLayer, FeatureMap};
use crate::alignment::solving::{LinearProgramming, Solver, SolvingSettings};
use crate::deconvolution::Deconvolution;
use std::sync::Arc;

pub struct Aligner {
    filter: Arc<dyn Filter>,
    solver: Arc<dyn Solver>,
}

impl Default for Aligner {
    fn default() -> Self {
        Self::new(
            FilteringSettings::default(),
            SolvingSettings::default(),
        )
    }
}

impl Aligner {
    pub fn new(
        filtering_settings: FilteringSettings,
        solving_settings: SolvingSettings
    ) -> Self {
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

        Self { filter, solver }
    }

    pub fn align_deconvolutions<D: AsRef<Deconvolution>>(&self, deconvolutions: &[D]) -> Alignment {
        let layer_count = deconvolutions.len();
        let mut feature_layers = deconvolutions
            .iter()
            .map(FeatureLayer::from)
            .collect::<Vec<_>>();
        let feature_maps = feature_layers[1..]
            .iter()
            .enumerate()
            .map(|(i, feature_layer)| {
                let assignments = self
                    .filter
                    .filter_assignments(&feature_layers[0], feature_layer);

                FeatureMap::new(0, i + 1, assignments)
            })
            .collect::<Vec<_>>();
        let solution = self.solver.solve(feature_maps);
        let mut chains = Vec::<AssignmentChain>::new();
        solution.iter().for_each(|feature_map| {
            let i = feature_map.layer_i();
            let j = feature_map.layer_j();
            feature_map
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
        chains.iter().for_each(|chain| {
            let first = chain.iter().next().unwrap();
            let maxp = feature_layers[*first.0].as_ref()[*first.1].maxp();
            chain.iter().skip(1).for_each(|(layer, feature)| {
                feature_layers[*layer].as_mut()[*feature].set_maxp(maxp);
            })
        });

        feature_layers
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
            })
            .collect()
    }
}
