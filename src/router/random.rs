use rand::Rng;

use crate::config::ModelId;
use crate::provider::provider::ResponseRequest;
use crate::router::router::{ModelInfo, Router};

pub struct RandomRouter {
    pub model_infos: Vec<ModelInfo>,
}

impl RandomRouter {
    pub fn new(model_infos: Vec<ModelInfo>) -> Self {
        Self { model_infos }
    }
}

impl Router for RandomRouter {
    fn name(&self) -> &'static str {
        "RandomRouter"
    }

    fn sample(&mut self, _input: &ResponseRequest) -> ModelId {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..self.model_infos.len());
        self.model_infos[idx].id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_router_sampling() {
        let model_infos = vec![
            ModelInfo {
                id: "model_x".to_string(),
                weight: 1,
            },
            ModelInfo {
                id: "model_y".to_string(),
                weight: 2,
            },
            ModelInfo {
                id: "model_z".to_string(),
                weight: 3,
            },
        ];
        let mut router = RandomRouter::new(model_infos.clone());
        let mut counts = std::collections::HashMap::new();

        for _ in 0..1000 {
            let sampled_id = router.sample(&ResponseRequest::default());
            *counts.entry(sampled_id.clone()).or_insert(0) += 1;
        }
        assert!(counts.len() == model_infos.len());
        for count in counts.values() {
            assert!(*count > 0);
        }
    }
}
