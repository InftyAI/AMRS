use std::str::FromStr;

use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;
use reqwest::header::HeaderName;

use crate::config::{ModelConfig, ModelId};
use crate::provider::provider::{APIError, Provider, ResponseRequest, ResponseResult};

pub struct OpenAIProvider {
    model: ModelId,
    config: OpenAIConfig,
    client: Option<Client<OpenAIConfig>>,
}

impl OpenAIProvider {
    pub fn new(config: &ModelConfig) -> Self {
        let api_key_var = format!(
            "{}_API_KEY",
            config.provider.as_ref().unwrap().to_uppercase()
        );
        let api_key = std::env::var(api_key_var).expect("API key environment variable not set");

        let openai_config = OpenAIConfig::new()
            .with_api_base(config.base_url.clone().unwrap())
            .with_api_key(api_key);

        OpenAIProvider {
            model: config.id.clone(),
            config: openai_config,
            client: None,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Result<Self, APIError> {
        let name = HeaderName::from_str(key)
            .map_err(|e| APIError::InvalidArgument(format!("Invalid header name: {}", e)))?;

        self.config = self.config.with_header(name, value)?;
        Ok(self)
    }

    pub fn build(mut self) -> Self {
        if self.client.is_none() {
            self.client = Some(Client::with_config(self.config.clone()));
        }
        self
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAIProvider"
    }

    async fn create_response(&self, request: ResponseRequest) -> Result<ResponseResult, APIError> {
        let client = self.client.as_ref().unwrap();
        client.responses().create(request).await
    }
}
