use rand::Rng;

use crate::config::ModelId;
use crate::provider::provider::ResponseRequest;
use crate::router::router::Router;

pub struct RandomRouter {
    pub model_ids: Vec<ModelId>,
}

impl RandomRouter {
    pub fn new(model_ids: Vec<ModelId>) -> Self {
        Self { model_ids }
    }
}

impl Router for RandomRouter {
    fn name(&self) -> &'static str {
        "RandomRouter"
    }

    fn sample(&self, _input: &ResponseRequest) -> ModelId {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..self.model_ids.len());
        self.model_ids[idx].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_router_sampling() {
        let model_ids = vec!["model_a".to_string(), "model_b".to_string()];
        let router = RandomRouter::new(model_ids.clone());
        let mut counts = std::collections::HashMap::new();
        for _ in 0..1000 {
            let sampled_id = router.sample(&ResponseRequest::default());
            *counts.entry(sampled_id.clone()).or_insert(0) += 1;
        }
        assert!(counts.len() == model_ids.len());
        for count in counts.values() {
            assert!(*count > 0);
        }
    }
}
