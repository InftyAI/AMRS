use std::str::FromStr;

use async_openai::types::responses::{
    AssistantRole, OutputItem, OutputMessage, OutputMessageContent, OutputStatus,
    OutputTextContent, Status,
};
use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;
use reqwest::header::HeaderName;

use crate::config::{ModelConfig, ModelId};
use crate::provider::provider::{
    APIError, CreateResponseReq, CreateResponseRes, Provider, validate_request,
};

pub struct FakeProvider {
    model: ModelId,
}

impl FakeProvider {
    pub fn new(config: ModelConfig) -> Self {
        Self {
            model: config.id.clone(),
        }
    }
}

#[async_trait]
impl Provider for FakeProvider {
    fn name(&self) -> &'static str {
        "FakeProvider"
    }

    async fn create_response(
        &self,
        request: CreateResponseReq,
    ) -> Result<CreateResponseRes, APIError> {
        validate_request(&request)?;

        Ok(CreateResponseRes {
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
