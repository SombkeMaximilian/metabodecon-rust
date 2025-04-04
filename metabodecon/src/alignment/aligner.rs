use crate::alignment::Alignment;
use crate::alignment::assignment::AssignmentChain;
use crate::alignment::feature::{FeatureLayer, FeatureMap};
use crate::alignment::solving::{LinearProgramming, Solver};
use crate::deconvolution::Deconvolution;
use std::sync::Arc;

pub struct Aligner {
    solver: Arc<dyn Solver>,
    max_distance: f64,
    min_similarity: f64,
}

impl Aligner {
    pub fn new(max_distance: f64, min_similarity: f64) -> Self {
        Self {
            solver: Arc::new(LinearProgramming),
            max_distance,
            min_similarity,
        }
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
                let assignments = feature_layers[0].assignment_candidates(
                    feature_layer,
                    self.max_distance,
                    self.min_similarity,
                );

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
