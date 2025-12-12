from amrs.config import ModelConfig, Config

def test_config():
    config = Config(
        base_url="https://api.example.com",
        temperature=0.7,
        models=[
            ModelConfig(
                id="openai:gpt-4",
                weight=20,
                temperature=0.5
            ),
            ModelConfig(
                id="anthropic:claude-2",
                weight=80,
                base_url="https://api.anthropic.com"
            )
        ],
        routing_mode="weighted",
        messages=[
            {"role": "user", "content": "Hello, how are you?"}
        ]
    )

    assert config.base_url == "https://api.example.com"
