use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;
use derive_builder::Builder;

use crate::client::config::{DEFAULT_PROVIDER, ModelConfig, ModelName};
use crate::provider::{common, provider};
use crate::types::error::OpenAIError;
use crate::types::{chat, responses};

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable", build_fn(skip))]
pub struct OpenAIProvider {
    model: ModelName,
    config: OpenAIConfig,
    client: Client<OpenAIConfig>,
    #[builder(default = "OPENAI_PROVIDER.to_string()", setter(custom))]
    provider_name: String,
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
            model: Some(config.name.clone()),
            config: Some(openai_config),
            client: None,
            provider_name: None,
        }
    }
}

impl OpenAIProviderBuilder {
    pub fn provider_name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.provider_name = Some(name.as_ref().to_string());
        self
    }

    pub fn build(&mut self) -> OpenAIProvider {
        OpenAIProvider {
            model: self.model.clone().unwrap(),
            config: self.config.clone().unwrap(),
            client: Client::with_config(self.config.as_ref().unwrap().clone()),
            provider_name: self
                .provider_name
                .clone()
                .unwrap_or(DEFAULT_PROVIDER.to_string()),
        }
    }
}

#[async_trait]
impl provider::Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAIProvider"
    }

    async fn create_completion(
        &self,
        request: chat::CreateChatCompletionRequest,
    ) -> Result<chat::CreateChatCompletionResponse, OpenAIError> {
        common::validate_completion_request(&request)?;

        // Set the model after validation since model is bind to the provider.
        let mut req = request.clone();
        req.model = self.model.clone();
        self.client.chat().create(req).await
    }

    async fn create_response(
        &self,
        request: responses::CreateResponse,
    ) -> Result<responses::Response, OpenAIError> {
        if !provider::RESPONSE_ENDPOINT_PROVIDERS.contains(&self.provider_name.as_str()) {
            return Err(OpenAIError::InvalidArgument(format!(
                "Provider '{}' doesn't support Responses endpoint",
                self.provider_name
            )));
        }

        common::validate_response_request(&request)?;

        // Set the model after validation since model is bind to the provider.
        let mut req = request.clone();
        req.model = Some(self.model.clone());
        self.client.responses().create(req).await
    }
}
