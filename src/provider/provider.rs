use async_openai::error::OpenAIError;
use async_openai::types::responses::{CreateResponse as OpenAIRequest, Response as OpenAIResponse};
use async_trait::async_trait;

use crate::config::ModelConfig;
use crate::provider::openai::OpenAIProvider;

pub type ResponseRequest = OpenAIRequest;
pub type ResponseResult = OpenAIResponse;
pub type APIError = OpenAIError;

pub fn construct_provider(config: &ModelConfig) -> Box<dyn Provider> {
    let provider = config.provider.as_ref().unwrap();
    match provider.as_str() {
        "openai" => Box::new(OpenAIProvider::new(config).build()),
        _ => panic!("Unsupported provider: {}", provider),
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn create_response(&self, request: ResponseRequest) -> Result<ResponseResult, APIError>;
}

// // test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     fn test_build_provider() {
//         struct TestCase {
//             name: &'static str,
//             config: ModelConfig,
//             expect_provider_type: &'static str,
//             error: bool,
//         }

//         let cases = vec![
//             TestCase {
//                 name: "OpenAI Provider",
//                 config: ModelConfig::new("test-model").with_provider("openai"),
//                 expect_provider_type: "OpenAIProvider",
//                 error: false,
//             },
//             // Add more test cases as needed
//         ];
//     }
