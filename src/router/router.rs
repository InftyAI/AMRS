use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use crate::config::{ModelConfig, ModelId, RoutingMode};
use crate::provider::provider::ResponseRequest;
use crate::router::random::RandomRouter;
use crate::router::weight::WeightedRouter;

pub fn build_router(mode: RoutingMode, models: Vec<ModelConfig>) -> Box<dyn Router> {
    let model_ids: Vec<ModelId> = models.iter().map(|m| m.id.clone()).collect();
    match mode {
        RoutingMode::Random => Box::new(RandomRouter::new(model_ids)),
        RoutingMode::Weighted => Box::new(WeightedRouter::new(model_ids)),
    }
}

pub trait Router {
    fn sample(&self, input: &ResponseRequest) -> &ModelId;
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
