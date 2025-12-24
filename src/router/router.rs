use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use crate::config::{ModelConfig, ModelId, RoutingMode};
use crate::provider::provider::CreateResponseReq;
use crate::router::random::RandomRouter;
use crate::router::wrr::WeightedRoundRobinRouter;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: ModelId,
    pub weight: i32,
}

pub fn construct_router(mode: RoutingMode, models: Vec<ModelConfig>) -> Box<dyn Router> {
    let model_infos: Vec<ModelInfo> = models
        .iter()
        .map(|m| ModelInfo {
            id: m.id.clone(),
            weight: m.weight.clone(),
        })
        .collect();
    match mode {
        RoutingMode::Random => Box::new(RandomRouter::new(model_infos)),
        RoutingMode::WRR => Box::new(WeightedRoundRobinRouter::new(model_infos)),
    }
}

pub trait Router {
    fn name(&self) -> &'static str;
    fn sample(&mut self, input: &CreateResponseReq) -> ModelId;
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

        let weighted_router = construct_router(RoutingMode::WRR, model_configs.clone());
        assert_eq!(weighted_router.name(), "WeightedRoundRobinRouter");
    }
}
