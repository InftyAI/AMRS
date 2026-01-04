use crate::client::config::{ModelConfig, ModelName, RouterMode};
use crate::router::random::RandomRouter;
use crate::router::wrr::WeightedRoundRobinRouter;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: ModelName,
    pub weight: i32,
}

pub fn construct_router(mode: RouterMode, models: Vec<ModelConfig>) -> Box<dyn Router> {
    let model_infos: Vec<ModelInfo> = models
        .iter()
        .map(|m| ModelInfo {
            name: m.name.clone(),
            weight: m.weight.clone(),
        })
        .collect();
    match mode {
        RouterMode::Random => Box::new(RandomRouter::new(model_infos)),
        RouterMode::WRR => Box::new(WeightedRoundRobinRouter::new(model_infos)),
    }
}

pub trait Router {
    fn name(&self) -> &'static str;
    fn sample(&self) -> ModelName;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_router_construction() {
        let model_configs = vec![
            ModelConfig::builder()
                .name("model_a".to_string())
                .provider(Some("openai".to_string()))
                .base_url(Some("https://api.openai.com/v1".to_string()))
                .build()
                .unwrap(),
            ModelConfig::builder()
                .name("model_b".to_string())
                .provider(Some("openai".to_string()))
                .base_url(Some("https://api.openai.com/v1".to_string()))
                .build()
                .unwrap(),
        ];

        let random_router = construct_router(RouterMode::Random, model_configs.clone());
        assert_eq!(random_router.name(), "RandomRouter");

        let weighted_router = construct_router(RouterMode::WRR, model_configs.clone());
        assert_eq!(weighted_router.name(), "WeightedRoundRobinRouter");
    }
}
