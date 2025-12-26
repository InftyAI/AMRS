use dotenvy::from_filename;

use arms::client;
use arms::types::responses;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_response() {
        from_filename(".env.integration-test").ok();

        // case 1: one model.
        let config = client::Config::builder()
            .provider("faker")
            .model(
                client::ModelConfig::builder()
                    .name("fake-model")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let mut client = client::Client::new(config);
        let request = responses::CreateResponseArgs::default()
            .input("tell me the weather today")
            .build()
            .unwrap();

        let response = client.create_response(request).await.unwrap();
        assert!(response.id.starts_with("fake-response-id"));
        assert!(response.model == "fake-model");

        // case 2: specify model in request.
        let config = client::Config::builder()
            .provider("openai")
            .model(
                client::ModelConfig::builder()
                    .name("gpt-3.5-turbo")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let mut client = client::Client::new(config);
        let request = responses::CreateResponseArgs::default()
            .model("gpt-3.5-turbo")
            .input("tell me a joke")
            .build()
            .unwrap();
        let response = client.create_response(request).await;
        assert!(response.is_err());

        // case 3: multiple models with router.
        let config = client::Config::builder()
            .provider("faker")
            .routing_mode(client::RoutingMode::WRR)
            .model(
                client::ModelConfig::builder()
                    .name("gpt-3.5-turbo")
                    .weight(1)
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
        let request = responses::CreateResponseArgs::default()
            .input("give me a poem about nature")
            .build()
            .unwrap();
        let _ = client.create_response(request).await.unwrap();
    }
}
