from enum import Enum
import os
from typing import Callable
from pydantic import BaseModel, Field, model_validator
from typing import List, Optional

# If provider not in the known list, doesn't mean it's invalid,
# but user must provide base_url.
class Provider(str, Enum):
    AMRS = "amrs"
    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    COHERE = "cohere"
    MISTRAL = "mistral"
    XAI = "xai"
    DEEPSEEK = "deepseek"
    DEEPINFRA = "deepinfra"
    OPENROUTER = "openrouter"

PROVIDER_BASE_URLS = {
    Provider.OPENAI: "https://api.openai.com/v1",
    # Provider.ANTHROPIC: "https://api.anthropic.com/v1",
    # Provider.COHERE: "https://api.cohere.ai",
    # Provider.MISTRAL: "https://api.mistral.ai/v1",
    # Provider.XAI: "https://api.x.ai/v1",
    # Provider.DEEPSEEK: "https://api.deepseek.ai/v1",
    Provider.DEEPINFRA: "https://api.deepinfra.com/v1/openai",
    Provider.OPENROUTER: "https://openrouter.ai/api/v1",
}


class BasicModelConfig(BaseModel):
    base_url: Optional[str] = Field(
        default=None, description="Base URL for the model API endpoint. If not provided, \
            the default base URL for the provider will be used. The priority of the base_url is: \
            model.base_url > global base_url > provider default base_url."
    )
    provider: Optional[str] = Field(
        default=Provider.AMRS, description="Provider name of the model. Default is 'amrs'. \
            Provider defines the API key environment variable name as <PROVIDER>_API_KEY."
    )
    temperature: Optional[float] = Field(
        default=0.8, description="Temperature setting for model generation."
    )
    max_tokens: Optional[int] = Field(
        default=1024, description="Maximum number of tokens for model generation."
    )


type ModelID = str

class ModelConfig(BasicModelConfig):
    id: ModelID = Field(
        description="ID of the model to be used."
    )
    weight: Optional[int] = Field(
        default=-1,
        description="Weight of the model for ensemble methods. Only used if routing_mode is 'weighted'.",
    )


class ChatRole(str, Enum):
    USER = "user"
    ASSISTANT = "assistant"
    SYSTEM = "system"


class Message(BaseModel):
    role: ChatRole = Field(description="Role of the message sender.")
    # For image messages, the format is different, but we only support text message for now.
    # See https://platform.openai.com/docs/api-reference/chat/create
    content: str = Field(description="Content of the message.")


class RoutingMode(str, Enum):
    RANDOM = "random"
    WEIGHTED = "weighted"


class Config(BasicModelConfig):
    models: List[ModelConfig] = Field(description="List of model configurations")
    routing_mode: RoutingMode = Field(
        default=RoutingMode.RANDOM,
        description="Routing mode for the model, default is random.",
    )
    callback_funcs: Optional[List[Callable]] = Field(
        default=None,
        description="Callback functions to be called after each model inference. Functions will be called sequentially.",
    )

    @model_validator(mode="after")
    def set_model_base_url(self):
        global_base_url = self.base_url

        for model in self.models:
            if not model.base_url:
                if model.provider in PROVIDER_BASE_URLS:
                    model.base_url = PROVIDER_BASE_URLS[model.provider]
                if global_base_url:
                    model.base_url = global_base_url
        return self

    @model_validator(mode="after")
    def validate_model_base_url(self):
        global_url_exist = self.base_url is not None

        for model in self.models:
            if model.provider not in PROVIDER_BASE_URLS and not model.base_url and not global_url_exist:
                raise ValueError(
                    f"Model '{model.id}' base_url is not provided."
                )

        return self

    @model_validator(mode="after")
    def validate_api_key(self):
        for model in self.models:
            api_key_env = f"{model.provider.upper()}_API_KEY"
            if api_key_env not in os.environ:
                raise ValueError(
                    f"API key for provider '{model.provider}' not found in environment variable '{api_key_env}'."
                )
        return self
