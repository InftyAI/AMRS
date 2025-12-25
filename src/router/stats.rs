use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::config::ModelName;

pub struct RouterStats {
    requests_per_model: HashMap<ModelName, AtomicUsize>,
}

impl RouterStats {
    pub fn default() -> Self {
        RouterStats {
            requests_per_model: HashMap::new(),
        }
    }

    pub fn increment_request(&mut self, model_id: &ModelName) -> usize {
        let counter = self
            .requests_per_model
            .entry(model_id.clone())
            .or_insert_with(|| AtomicUsize::new(0));
        counter.fetch_add(1, Ordering::Relaxed)
    }
}
