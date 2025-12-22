use std::collections::HashMap;

use crate::config::{Config, ModelConfig, ModelId, RoutingMode};
use crate::provider::provider;
use crate::router::router;

pub struct Client {
    router_tracker: Option<router::RouterTracker>,
    router: Box<dyn router::Router>,
    providers: HashMap<ModelId, Box<dyn provider::Provider>>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        let mut cfg = config.clone();
        cfg.populate();

        let providers = cfg
            .models
            .iter()
            .map(|m| (m.id.clone(), provider::construct_provider(m)))
            .collect();

        Self {
            router_tracker: None,
            providers: providers,
            router: router::construct_router(cfg.routing_mode, cfg.models),
        }
    }

    pub fn enable_router_tracker(&mut self) {
        if self.router_tracker.is_none() {
            self.router_tracker = Some(router::RouterTracker::new());
        }
    }

    pub async fn create_response(
        &self,
        request: provider::ResponseRequest,
    ) -> Result<provider::ResponseResult, provider::APIError> {
        let model_id = self.router.sample(&request);
        let provider = self.providers.get(&model_id).unwrap();
        provider.create_response(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_client_new() {
        struct TestCase {
            name: &'static str,
            config: Config,
            expected_router_name: &'static str,
            enabled_tracker: bool,
        }

        let cases = vec![
            TestCase {
                name: "basic config",
                config: Config::builder()
                    .models(vec![
                        ModelConfig::builder()
                            .id("model_c".to_string())
                            .build()
                            .unwrap(),
                    ])
                    .build()
                    .unwrap(),
                expected_router_name: "RandomRouter",
                enabled_tracker: false,
            },
            TestCase {
                name: "weighted router",
                config: Config::builder()
                    .routing_mode(RoutingMode::Weighted)
                    .models(vec![
                        crate::config::ModelConfig::builder()
                            .id("model_a".to_string())
                            .provider(Some("openai".to_string()))
                            .base_url(Some("https://api.openai.com/v1".to_string()))
                            .weight(1)
                            .build()
                            .unwrap(),
                        crate::config::ModelConfig::builder()
                            .id("model_b".to_string())
                            .provider(Some("openai".to_string()))
                            .base_url(Some("https://api.openai.com/v1".to_string()))
                            .weight(3)
                            .build()
                            .unwrap(),
                    ])
                    .build()
                    .unwrap(),
                expected_router_name: "WeightedRouter",
                enabled_tracker: false,
            },
            TestCase {
                name: "router tracker enabled",
                config: Config::builder()
                    .models(vec![
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
                    ])
                    .build()
                    .unwrap(),
                expected_router_name: "RandomRouter",
                enabled_tracker: true,
            },
        ];

        for case in cases {
            let mut client = Client::new(case.config.clone());
            if case.enabled_tracker {
                client.enable_router_tracker();
            }
            assert_eq!(
                client.router.name(),
                case.expected_router_name,
                "Test case '{}' failed",
                case.name
            );
            assert_eq!(
                client.router_tracker.is_some(),
                case.enabled_tracker,
                "Test case '{}' failed on router tracker state",
                case.name
            );
            assert_eq!(
                client.providers.len(),
                case.config.models.len(),
                "Test case '{}' failed on providers count",
                case.name
            );
        }
    }
}
