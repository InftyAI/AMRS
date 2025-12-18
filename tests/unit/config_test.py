import os

from amrs.config import ModelConfig, Config


def test_model_config():
    test_cases = [
        {
            "name": "valid with default config",
            "input": {
                "id": "gpt-4",
                "base_url": "https://api.example.com/v1",
            },
            "expected": lambda: Config(
                models=[ModelConfig(id="gpt-4", provider="amrs", base_url="https://api.example.com/v1")],
            ),
            "fail": False,
        },
        {
            "name": "valid with custom config and default base_url",
            "input": {
                "id": "gpt-4",
                "provider": "openai",
                "temperature": 0.5,
                "max_tokens": 1500,
            },
            "env": {"OPENAI_API_KEY": "test_openai_key"},
            "expected": lambda: Config(
                models=[
                    ModelConfig(
                        id="gpt-4",
                        provider="openai",
                        temperature=0.5,
                        max_tokens=1500,
                        base_url="https://api.openai.com/v1",
                    )
                ],
            ),
            "fail": False,
        },
        {
            "name": "valid in-house provider with custom base_url",
            "input": {
                "id": "gpt-4",
                "provider": "openai",
                "base_url": "https://custom.api.endpoint/v1",
            },
            "env": {"OPENAI_API_KEY": "custom_key"},
            "expected": lambda: Config(
                models=[
                    ModelConfig(
                        id="gpt-4",
                        provider="openai",
                        base_url="https://custom.api.endpoint/v1",
                    )
                ],
            ),
            "fail": False,
        },
        {
            "name": "valid custom_provider with custom base_url",
            "input": {
                "id": "custom-model",
                "provider": "custom_provider",
                "base_url": "https://custom.api.endpoint/v1",
            },
            "env": {"CUSTOM_PROVIDER_API_KEY": "custom_key"},
            "expected": lambda: Config(
                models=[
                    ModelConfig(
                        id="custom-model",
                        provider="custom_provider",
                        base_url="https://custom.api.endpoint/v1",
                    )
                ],
            ),
            "fail": False,
        },
        {
            "name": "invalid unknown_provider with missing env var",
            "input": {
                "id": "gpt-4",
                "provider": "unknown_provider",
                "base_url": "https://custom.api.endpoint/v1",
            },
            "expected": None,
            "fail": True,
        },
        {
            "name": "invalid unknown_provider without base_url",
            "input": {
                "id": "gpt-4",
                "provider": "unknown_provider",
            },
            "env": {"UNKNOWN_PROVIDER_API_KEY": "some_key"},
            "expected": None,
            "fail": True,
        }
    ]

    for case in test_cases:
        if "env" in case:
            for key, value in case["env"].items():
                os.environ[key] = value

        if case["fail"]:
            try:
                Config(models=[ModelConfig(**case["input"])])
                assert False, (
                    f"Test case '{case['name']}' should have failed but didn't."
                )
            except ValueError:
                pass  # Expected to fail
        else:
            config = Config(models=[ModelConfig(**case["input"])])
            assert config == case["expected"](), (
                f"Test case '{case['name']}' failed."
            )

        # reset
        if "env" in case:
            for key in case["env"].keys():
                del os.environ[key]
