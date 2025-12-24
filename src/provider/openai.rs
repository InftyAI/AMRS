use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;
use derive_builder::Builder;

use crate::config::{ModelConfig, ModelId};
use crate::provider::provider::{
    APIError, CreateResponseReq, CreateResponseRes, Provider, validate_request,
};

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable", build_fn(skip))]
pub struct OpenAIProvider {
    model: ModelId,
    config: OpenAIConfig,
    client: Client<OpenAIConfig>,
}

impl OpenAIProvider {
    pub fn builder(config: ModelConfig) -> OpenAIProviderBuilder {
        let api_key_var = format!(
            "{}_API_KEY",
            config.provider.as_ref().unwrap().to_uppercase()
        );
        let api_key = std::env::var(api_key_var).expect("API key environment variable not set");

        let openai_config = OpenAIConfig::new()
            .with_api_base(config.base_url.clone().unwrap())
            .with_api_key(api_key);

        OpenAIProviderBuilder {
            model: Some(config.id.clone()),
            config: Some(openai_config),
            client: None,
        }
    }
}

impl OpenAIProviderBuilder {
    pub fn build(&mut self) -> OpenAIProvider {
        OpenAIProvider {
            model: self.model.clone().unwrap(),
            config: self.config.clone().unwrap(),
            client: Client::with_config(self.config.as_ref().unwrap().clone()),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAIProvider"
    }

    async fn create_response(
        &self,
        request: CreateResponseReq,
    ) -> Result<CreateResponseRes, APIError> {
        validate_request(&request)?;
        self.client.responses().create(request).await
    }
}
