# AMRS

 [![Latest Release](https://img.shields.io/github/v/release/inftyai/amrs?include_prereleases)](https://github.com/inftyai/amrs/releases/latest)

The Adaptive Model Routing System (AMRS) is a framework designed to select the best-fit model for exploration and exploitation. (still under development)

Thanks to [async-openai](https://github.com/64bit/async-openai), AMRS builds on top of it to provide adaptive model routing capabilities.

## Features

- Flexible routing strategies, including:
  - **Random**: Randomly selects a model from the available models.
  - **WRR**: Weighted Round Robin selects models based on predefined weights.
  - **UCB1**: Upper Confidence Bound based model selection (coming soon).
  - **Adaptive**: Dynamically selects models based on performance metrics (coming soon).

- Broad provider support:
  - OpenAI compatible providers (DeepInfra, OpenRouter, etc.)
  - More on the way

## How to use

Here's a simple example with the Weighted Round Robin (WRR) routing mode:


```rust
// Before running the code, make sure to set your OpenAI API key in the environment variable:
// export OPENAI_API_KEY="your_openai_api_key"

use tokio::runtime::Runtime;
use arms::client;
use arms::types::chat;

let config = client::Config::builder()
    .provider("openai")
    .routing_mode(client::RoutingMode::WRR)
    .model(
        client::ModelConfig::builder()
            .name("gpt-3.5-turbo")
            .weight(2)
            .build()
            .unwrap(),
    )
    .model(
        client::ModelConfig::builder()
            .name("gpt-4")
            .weight(1)
            .build()
            .unwrap(),
    )
    .build()
    .unwrap();

let mut client = client::Client::new(config);
let request = chat::CreateChatCompletionRequestArgs::default()
    .messages([
        chat::ChatCompletionRequestSystemMessage::from("You are a helpful assistant.").into(),
        chat::ChatCompletionRequestUserMessage::from("How is the weather today?").into(),
    ])
    .build()
    .unwrap();

let result = Runtime::new().unwrap().block_on(client.create_completion(request));
```

## Contributing

🚀 All kinds of contributions are welcomed ! Please follow [Contributing](/CONTRIBUTING.md).

[![Star History Chart](https://api.star-history.com/svg?repos=inftyai/amrs&type=Date)](https://www.star-history.com/#inftyai/amrs&Date)