from enum import Enum
from typing import Callable
from pydantic import BaseModel, Field, model_validator
from typing import List, Optional

class BasicModelConfig(BaseModel):
    base_url: Optional[str] = Field(
        default=None,
        description="Global base URL for model API endpoints."
    )
    temperature: Optional[float] = Field(
        default=0.8,
        description="Global temperature setting for model generation."
    )

class ModelConfig(BasicModelConfig):
    id: str = Field(
        description="ID of the model, including both the provider name and the model name, e.g. 'openai:gpt-4'."
    )
    weight: Optional[int] = Field(
        default=1,
        description="Weight of the model for ensemble methods. Only used if routing_mode is 'weighted'."
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
    messages: str | List[Message] = Field(
        description="Messages to be sent to the model(s). Can be a string or a list of Message objects."
    )

    @model_validator(mode="after")
    def ensure_at_least_one_base_url(self):
        global_url_exist =  self.base_url is not None

        for model in self.models:
            if not model.base_url and not global_url_exist:
                raise ValueError("At least one base_url must be specified either in the global config or in each model config.")

        return self
