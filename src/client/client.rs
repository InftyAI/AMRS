use std::collections::HashMap;

use crate::config::Config;
use crate::config::ModelId;
use crate::provider::provider;
use crate::router::router;

// ------------------ Chat Role ------------------
#[derive(Debug, Clone)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

// ------------------ Message ------------------
#[derive(Debug, Clone)]
pub struct TextMessage {
    pub role: ChatRole,
    pub content: String,
}

pub struct Client {
    config: Config,
    router_tracker: Option<router::RouterTracker>,
    router: Box<dyn router::Router>,
    providers: HashMap<ModelId, Box<dyn provider::Provider>>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        let mut cfg = config;
        cfg.finalize().expect("Invalid configuration");

        let providers = cfg
            .models
            .iter()
            .map(|m| {
                let provider = m
                    .provider
                    .as_ref()
                    .expect("Model provider must be specified");

                (m.id.clone(), provider::build_provider(provider, m))
            })
            .collect();

        Self {
            config: cfg.clone(),
            router_tracker: None,
            providers: providers,
            router: router::build_router(cfg.routing_mode, cfg.models),
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
        let provider = self.providers.get(model_id).unwrap();
        provider.create_response(request).await
    }
}
