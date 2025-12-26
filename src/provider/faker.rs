use async_trait::async_trait;

use crate::client::config::{ModelConfig, ModelName};
use crate::provider::provider;
use crate::types::error::OpenAIError;
use crate::types::responses::{
    AssistantRole, CreateResponse, OutputItem, OutputMessage, OutputMessageContent, OutputStatus,
    OutputTextContent, Response, Status,
};

pub struct FakerProvider {
    model: ModelName,
}

impl FakerProvider {
    pub fn new(config: ModelConfig) -> Self {
        Self {
            model: config.name.clone(),
        }
    }
}

#[async_trait]
impl provider::Provider for FakerProvider {
    fn name(&self) -> &'static str {
        "FakeProvider"
    }

    async fn create_response(&self, request: CreateResponse) -> Result<Response, OpenAIError> {
        provider::validate_responses_request(&request)?;

        Ok(Response {
            id: "fake-response-id".to_string(),
            object: "text_completion".to_string(),
            model: self.model.clone(),
            usage: None,
            output: vec![OutputItem::Message(OutputMessage {
                id: "fake-message-id".to_string(),
                status: OutputStatus::Completed,
                role: AssistantRole::Assistant,
                content: vec![OutputMessageContent::OutputText(OutputTextContent {
                    annotations: vec![],
                    logprobs: None,
                    text: "This is a fake response.".to_string(),
                })],
            })],
            created_at: 1_600_000_000,
            background: None,
            billing: None,
            conversation: None,
            error: None,
            incomplete_details: None,
            instructions: None,
            max_output_tokens: None,
            metadata: None,
            prompt: None,
            parallel_tool_calls: None,
            previous_response_id: None,
            prompt_cache_key: None,
            prompt_cache_retention: None,
            reasoning: None,
            safety_identifier: None,
            service_tier: None,
            status: Status::Completed,
            temperature: None,
            text: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            top_logprobs: None,
            truncation: None,
        })
    }
}
