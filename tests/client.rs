use dotenvy::from_filename;

use arms::{Client, Config, CreateResponseArgs, ModelConfig, RoutingMode};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_response() {
        from_filename(".env.integration-test").ok();

        // case 1: one model.
        let config = Config::builder()
            .provider("fake")
            .model(ModelConfig::builder().name("fake-model").build().unwrap())
            .build()
            .unwrap();

        let mut client = Client::new(config);
        let request = CreateResponseArgs::default()
            .input("tell me the weather today")
            .build()
            .unwrap();

        let response = client.create_response(request).await.unwrap();
        assert!(response.id.starts_with("fake-response-id"));
        assert!(response.model == "fake-model");

        // case 2: specify model in request.
        let config = Config::builder()
            .provider("openai")
            .model(
                ModelConfig::builder()
                    .name("gpt-3.5-turbo")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let mut client = Client::new(config);
        let request = CreateResponseArgs::default()
            .model("gpt-3.5-turbo")
            .input("tell me a joke")
            .build()
            .unwrap();
        let response = client.create_response(request).await;
        assert!(response.is_err());

        // case 3: multiple models with router.
        let config = Config::builder()
            .provider("fake")
            .routing_mode(RoutingMode::WRR)
            .model(
                ModelConfig::builder()
                    .name("gpt-3.5-turbo")
                    .weight(1)
                    .build()
                    .unwrap(),
            )
            .model(
                ModelConfig::builder()
                    .name("gpt-4")
                    .weight(1)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let mut client = Client::new(config);
        let request = CreateResponseArgs::default()
            .input("give me a poem about nature")
            .build()
            .unwrap();
        let _ = client.create_response(request).await.unwrap();
    }
}
