use async_openai::error::OpenAIError as OpenAI_Error;
use async_openai::types::responses::{
    CreateResponse, CreateResponseArgs as OpenAICreateResponseArgs, Response,
};
use async_trait::async_trait;

use crate::config::ModelConfig;
use crate::provider::fake::FakeProvider;
use crate::provider::openai::OpenAIProvider;

pub type CreateResponseReq = CreateResponse;
pub type CreateResponseArgs = OpenAICreateResponseArgs;
pub type CreateResponseRes = Response;
pub type APIError = OpenAI_Error;

pub fn construct_provider(config: ModelConfig) -> Box<dyn Provider> {
    let provider = config.provider.as_ref().unwrap();
    match provider.to_uppercase().as_ref() {
        "FAKE" => Box::new(FakeProvider::new(config)),
        "OPENAI" => Box::new(OpenAIProvider::builder(config).build()),
        _ => panic!("Unsupported provider: {}", provider),
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn create_response(
        &self,
        request: CreateResponseReq,
    ) -> Result<CreateResponseRes, APIError>;
}

pub fn validate_request(request: &CreateResponseReq) -> Result<(), APIError> {
    if request.model.is_some() {
        return Err(APIError::InvalidArgument(
            "Model ID must be specified in the config".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_construction() {
        struct TestCase {
            name: &'static str,
            config: ModelConfig,
            expect_provider_type: &'static str,
        }

        let cases = vec![
            TestCase {
                name: "OpenAI Provider",
                config: ModelConfig::builder()
                    .id("test-model".to_string())
                    .provider(Some("openai".to_string()))
                    .base_url(Some("https://api.openai.com/v1".to_string()))
                    .build()
                    .unwrap(),
                expect_provider_type: "OpenAIProvider",
            },
            TestCase {
                name: "Unsupported Provider",
                config: ModelConfig::builder()
                    .id("test-model".to_string())
                    .provider(Some("unsupported".to_string()))
                    .base_url(Some("https://api.openai.com/v1".to_string()))
                    .build()
                    .unwrap(),
                expect_provider_type: "",
            },
        ];

        for case in cases {
            if case.expect_provider_type.is_empty() {
                let result = std::panic::catch_unwind(|| {
                    construct_provider(case.config);
                });
                assert!(
                    result.is_err(),
                    "Test case '{}' did not panic as expected",
                    case.name
                );
            } else {
                let provider = construct_provider(case.config);
                assert!(
                    provider.name() == case.expect_provider_type,
                    "Test case '{}': expected provider type '{}', got '{}'",
                    case.name,
                    case.expect_provider_type,
                    provider.name()
                );
            }
        }
    }
}
