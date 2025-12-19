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
    fn sample(&self, _input: &ResponseRequest) -> &ModelId {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..self.model_ids.len());
        &self.model_ids[idx]
    }
}
