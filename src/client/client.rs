use std::collections::HashMap;

use crate::config::{Config, ModelId};
use crate::provider::provider;
use crate::router::router;

pub struct Client {
    providers: HashMap<ModelId, Box<dyn provider::Provider>>,
    router: Box<dyn router::Router>,
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
            providers: providers,
            router: router::construct_router(cfg.routing_mode, cfg.models),
        }
    }

    pub async fn create_response(
        &mut self,
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
    use crate::config::{Config, ModelConfig, RoutingMode};
    use dotenvy::from_filename;

    #[test]
    fn test_client_new() {
        from_filename(".env.test").ok();

        struct TestCase {
            name: &'static str,
            config: Config,
            expected_router_name: &'static str,
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
            },
            TestCase {
                name: "weighted round-robin router",
                config: Config::builder()
                    .routing_mode(RoutingMode::WRR)
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
                expected_router_name: "WeightedRoundRobinRouter",
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
            },
        ];

        for case in cases {
            let client = Client::new(case.config.clone());
            assert_eq!(
                client.router.name(),
                case.expected_router_name,
                "Test case '{}' failed",
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
