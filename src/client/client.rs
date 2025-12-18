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
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            router_tracker: None,
            providers: HashMap::new(),
            router: router::build_router(&config.routing_mode, &config.models),
        }
    }

    pub fn enable_router_tracker(&mut self) {
        if self.router_tracker.is_none() {
            self.router_tracker = Some(router::RouterTracker::new());
        }
    }

    pub fn build(&mut self) {
        self.config.models.iter().for_each(|m| {
            self.providers.insert(
                m.id.clone(),
                provider::build_provider(&m.provider.as_ref().unwrap(), m),
            );
        });
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
