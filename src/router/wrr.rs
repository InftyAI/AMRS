use crate::router::router::{ModelInfo, Router};
use crate::{config::ModelId, provider::provider::ResponseRequest};

pub struct WeightedRoundRobinRouter {
    total_weight: i32,
    model_infos: Vec<ModelInfo>,
    // current_weight is ordered by model_infos index.
    current_weights: Vec<i32>,
}

impl WeightedRoundRobinRouter {
    pub fn new(model_infos: Vec<ModelInfo>) -> Self {
        let total_weight = model_infos.iter().map(|m| m.weight).sum();
        let length = model_infos.len();

        Self {
            model_infos: model_infos,
            total_weight: total_weight,
            current_weights: vec![0; length],
        }
    }
}

impl Router for WeightedRoundRobinRouter {
    fn name(&self) -> &'static str {
        "WeightedRoundRobinRouter"
    }

    // Use Smooth Weighted Round Robin Algorithm.
    fn sample(&mut self, _input: &ResponseRequest) -> ModelId {
        // return early if only one model.
        if self.model_infos.len() == 1 {
            return self.model_infos[0].id.clone();
        }

        self.current_weights
            .iter_mut()
            .enumerate()
            .for_each(|(i, weight)| {
                *weight += self.model_infos[i].weight;
            });

        let mut max_index = 0;
        for i in 1..self.current_weights.len() {
            if self.current_weights[i] > self.current_weights[max_index] {
                max_index = i;
            }
        }

        self.current_weights[max_index] -= self.total_weight;
        self.model_infos[max_index].id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_weighted_round_robin_sampling() {
        let model_infos = vec![
            ModelInfo {
                id: "model_x".to_string(),
                weight: 1,
            },
            ModelInfo {
                id: "model_y".to_string(),
                weight: 3,
            },
            ModelInfo {
                id: "model_z".to_string(),
                weight: 6,
            },
        ];
        let mut wrr = WeightedRoundRobinRouter::new(model_infos.clone());
        let mut counts = HashMap::new();
        for _ in 0..1000 {
            let sampled_id = wrr.sample(&ResponseRequest::default());
            *counts.entry(sampled_id.clone()).or_insert(0) += 1;
        }
        assert!(counts.len() == model_infos.len());
        // Check approximate distribution.
        let total_counts: usize = counts.values().sum();
        assert!(total_counts == 1000);
        let model_x_counts = *counts.get("model_x").unwrap_or(&0);
        let model_y_counts = *counts.get("model_y").unwrap_or(&0);
        let model_z_counts = *counts.get("model_z").unwrap_or(&0);
        let model_x_ratio = model_x_counts as f64 / total_counts as f64;
        let model_y_ratio = model_y_counts as f64 / total_counts as f64;
        let model_z_ratio = model_z_counts as f64 / total_counts as f64;
        assert!((model_x_ratio - 0.1).abs() < 0.05);
        assert!((model_y_ratio - 0.3).abs() < 0.05);
        assert!((model_z_ratio - 0.6).abs() < 0.05);
    }
}
