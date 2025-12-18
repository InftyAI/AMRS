use async_openai::error::OpenAIError;
use async_openai::types::responses::{CreateResponse as OpenAIRequest, Response as OpenAIResponse};
use async_trait::async_trait;

use crate::config::{ModelConfig, ProviderName};
use crate::provider::openai::OpenAIProvider;

pub type ResponseRequest = OpenAIRequest;
pub type ResponseResult = OpenAIResponse;
pub type APIError = OpenAIError;

pub fn build_provider(provider: &ProviderName, config: &ModelConfig) -> Box<dyn Provider> {
    match provider.as_str() {
        "openai" => Box::new(OpenAIProvider::new(config).build()),
        _ => panic!("Unsupported provider: {}", provider),
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn create_response(&self, request: ResponseRequest) -> Result<ResponseResult, APIError>;
}
