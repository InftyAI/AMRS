use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use crate::config::{ModelConfig, ModelId, RoutingMode};
use crate::provider::provider::ResponseRequest;
use crate::router::random::RandomRouter;
use crate::router::weight::WeightedRouter;

pub fn construct_router(mode: RoutingMode, models: Vec<ModelConfig>) -> Box<dyn Router> {
    let model_ids: Vec<ModelId> = models.iter().map(|m| m.id.clone()).collect();
    match mode {
        RoutingMode::Random => Box::new(RandomRouter::new(model_ids)),
        RoutingMode::Weighted => Box::new(WeightedRouter::new(model_ids)),
    }
}

pub trait Router {
    fn name(&self) -> &'static str;
    fn sample(&self, input: &ResponseRequest) -> ModelId;
}

pub struct RouterTracker {
    total_requests: HashMap<ModelId, AtomicUsize>,
    avg_latencies: HashMap<ModelId, AtomicUsize>,
    total_tokens: HashMap<ModelId, AtomicUsize>,
}

impl RouterTracker {
    pub fn new() -> Self {
        RouterTracker {
            total_requests: HashMap::new(),
            avg_latencies: HashMap::new(),
            total_tokens: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_router_construction() {
        let model_configs = vec![
            ModelConfig::builder()
                .id("model_a".to_string())
                .provider(Some("openai".to_string()))
                .base_url(Some("https://api.openai.com/v1".to_string()))
                .build()
                .unwrap(),
            ModelConfig::builder()
                .id("model_b".to_string())
                .provider(Some("openai".to_string()))
                .base_url(Some("https://api.openai.com/v1".to_string()))
                .build()
                .unwrap(),
        ];

        let random_router = construct_router(RoutingMode::Random, model_configs.clone());
        assert_eq!(random_router.name(), "RandomRouter");

        let weighted_router = construct_router(RoutingMode::Weighted, model_configs.clone());
        assert_eq!(weighted_router.name(), "WeightedRouter");
    }
}
