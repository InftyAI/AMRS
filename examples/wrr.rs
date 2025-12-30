use arms::client;
use arms::types::chat;
use tokio::runtime::Runtime;

fn main() {
    let config = client::Config::builder()
        .provider("deepinfra")
        .routing_mode(client::RouterMode::WRR)
        .model(
            client::ModelConfig::builder()
                .name("deepseek-ai/DeepSeek-V3.2")
                .weight(2)
                .build()
                .unwrap(),
        )
        .model(
            client::ModelConfig::builder()
                .name("nvidia/Nemotron-3-Nano-30B-A3B")
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
