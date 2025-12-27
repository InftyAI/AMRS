use async_trait::async_trait;

use crate::client::config::ModelConfig;
use crate::provider::faker::FakerProvider;
use crate::provider::openai::OpenAIProvider;
use crate::types::error::OpenAIError;
use crate::types::{chat, responses};

// Not all providers support response endpoint.
pub const RESPONSE_ENDPOINT_PROVIDERS: &[&str] = &["FAKER", "OPENAI"];

pub fn construct_provider(config: ModelConfig) -> Box<dyn Provider> {
    let provider = config.provider.clone().unwrap();

    match provider.to_uppercase().as_ref() {
        "FAKER" => Box::new(FakerProvider::new(config)),
        "OPENAI" | "DEEPINFRA" => Box::new(
            OpenAIProvider::builder(config)
                .provider_name(provider)
                .build(),
        ),
        _ => panic!("Unsupported provider: {}", provider),
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    // Used in tests only now.
    fn name(&self) -> &'static str;
    async fn create_response(
        &self,
        request: responses::CreateResponse,
    ) -> Result<responses::Response, OpenAIError>;
    async fn create_completion(
        &self,
        request: chat::CreateChatCompletionRequest,
    ) -> Result<chat::CreateChatCompletionResponse, OpenAIError>;
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
                    .name("test-model".to_string())
                    .provider(Some("openai".to_string()))
                    .base_url(Some("https://api.openai.com/v1".to_string()))
                    .build()
                    .unwrap(),
                expect_provider_type: "OpenAIProvider",
            },
            TestCase {
                name: "Unsupported Provider",
                config: ModelConfig::builder()
                    .name("test-model".to_string())
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
