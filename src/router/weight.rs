use super::router::Router;
use crate::{config::ModelId, provider::provider::ResponseRequest};

pub struct WeightedRouter {
    pub model_ids: Vec<ModelId>,
}

impl WeightedRouter {
    pub fn new(model_ids: &[ModelId]) -> Self {
        Self {
            model_ids: model_ids.to_vec(),
        }
    }
}

impl Router for WeightedRouter {
    fn sample(&self, _input: &ResponseRequest) -> &ModelId {
        // TODO: Implement weighted sampling logic
        return &self.model_ids[0];
    }
}
