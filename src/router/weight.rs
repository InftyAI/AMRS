use super::router::Router;
use crate::{config::ModelId, provider::provider::ResponseRequest};

pub struct WeightedRouter {
    pub model_ids: Vec<ModelId>,
}

impl WeightedRouter {
    pub fn new(model_ids: Vec<ModelId>) -> Self {
        Self { model_ids }
    }
}

impl Router for WeightedRouter {
    fn sample(&self, _input: &ResponseRequest) -> &ModelId {
        // TODO: Implement weighted sampling logic
        return &self.model_ids[0];
    }
}
