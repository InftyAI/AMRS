use tokio::runtime::Runtime;

use arms::client;
use arms::types::responses;

fn main() {
    let config = client::Config::builder()
        .provider("deepinfra")
        .routing_mode(client::RoutingMode::WRR)
        .model(
            client::ModelConfig::builder()
                .name("nvidia/Nemotron-3-Nano-30B-A3B")
                .weight(1)
                .build()
                .unwrap(),
        )
        .model(
            client::ModelConfig::builder()
                .name("deepseek-ai/DeepSeek-V3.2")
                .weight(2)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let mut client = client::Client::new(config);

    let request = responses::CreateResponseArgs::default()
        .input(responses::InputParam::Items(vec![
            responses::InputItem::EasyMessage(responses::EasyInputMessage {
                r#type: responses::MessageType::Message,
                role: responses::Role::User,
                content: responses::EasyInputContent::Text("What is AGI?".to_string()),
            }),
        ]))
        .build()
        .unwrap();

    let result = Runtime::new()
        .unwrap()
        .block_on(client.create_response(request));

    match result {
        Ok(response) => {
            println!("Response ID: {}", response.id);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
