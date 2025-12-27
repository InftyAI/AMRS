use crate::types::error::OpenAIError;
use crate::types::{chat, responses};

pub fn validate_completion_request(
    request: &chat::CreateChatCompletionRequest,
) -> Result<(), OpenAIError> {
    if request.model != "" {
        return Err(OpenAIError::InvalidArgument(
            "Model must be specified in the client.Config".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_response_request(request: &responses::CreateResponse) -> Result<(), OpenAIError> {
    if request.model.is_some() {
        return Err(OpenAIError::InvalidArgument(
            "Model must be specified in the client.Config".to_string(),
        ));
    }
    Ok(())
}
