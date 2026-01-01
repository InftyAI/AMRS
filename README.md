# AMRS

 [![Latest Release](https://img.shields.io/github/v/release/inftyai/amrs?include_prereleases)](https://github.com/inftyai/amrs/releases/latest)

The Adaptive Model Routing System (AMRS) is a framework designed to select the best-fit model for exploration and exploitation. Rust core with python bindings. Still under active development 🚧.

AMRS builds on top of [async-openai](https://github.com/64bit/async-openai) to provide API services for quick setup. Thanks to open source 💙.

## Features

- **Endpoints Support**:
  - Chat Completions
  - Responses
  - More on the way

- **Flexible Routing Strategies**:
  - **Random(default)**: Randomly selects a model from the available models.
  - **WRR**: Weighted Round Robin selects models based on predefined weights.
  - **UCB1**: Upper Confidence Bound for balancing exploration and exploitation (coming soon).
  - **Adaptive**: Dynamically selects models based on performance metrics (coming soon).

- **Various Providers Support**:
  - OpenAI compatible providers (OpenAI, DeepInfra, etc.)
  - More on the way

## How to Install

Run the following Cargo command in your project directory:

`cargo add arms`

Or add the following line to your Cargo.toml:

`arms = "0.0.1"`

## How to Use

Here's a simple example with the Weighted Round Robin (WRR) router mode. Before running the code, make sure to set your provider API key in the environment variable by running `export <PROVIDER>_API_KEY="your_provider_api_key"`.
Here we use OpenAI as an example.


```rust
// Make sure OPENAI_API_KEY is set in your environment variables before running this code.

use arms::client;
use arms::types::chat;
use tokio::runtime::Runtime;

fn main() {
    let config = client::Config::builder()
        .provider("openai")
        .router_mode(client::RouterMode::WRR)
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
            chat::ChatCompletionRequestUserMessage::from("How long it takes to learn Rust?").into(),
        ])
        .build()
        .unwrap();

    let result = Runtime::new()
        .unwrap()
        .block_on(client.create_completion(request));
    match result {
        Ok(response) => {
            for choice in response.choices {
                println!("Response: {:?}", choice.message.content);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

See more examples [here](/examples) folder.

## Contributing

🚀 All kinds of contributions are welcomed ! Please follow [Contributing](/CONTRIBUTING.md).

[![Star History Chart](https://api.star-history.com/svg?repos=inftyai/amrs&type=Date)](https://www.star-history.com/#inftyai/amrs&Date)
