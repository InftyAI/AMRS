use std::collections::HashMap;
use std::env;

use lazy_static::lazy_static;

// ------------------ Provider ------------------
pub type ProviderName = String;
const AMRS_PROVIDER: &str = "AMRS";

lazy_static! {
    pub static ref PROVIDER_BASE_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("OPENAI", "https://api.openai.com/v1");
        m.insert("DEEPINFRA", "https://api.deepinfra.com/v1/openai");
        m.insert("OPENROUTER", "https://openrouter.ai/api/v1");
        // TODO: support more providers here...
        m
    };
}

// ------------------ Routing Mode ------------------
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingMode {
    Random,
    Weighted,
}

// ------------------ Model Config ------------------
pub type ModelId = String;

#[derive(Debug, Clone)]
pub struct ModelConfig {
    // model-specific configs, will override global configs if provided
    pub base_url: Option<String>,
    pub provider: Option<ProviderName>,
    pub temperature: Option<f32>,
    pub max_output_tokens: Option<usize>,

    pub id: ModelId,
    pub weight: i32, // -1 if unused
}

impl ModelConfig {
    fn new(id: ModelId) -> Self {
        Self {
            base_url: None,
            provider: None,
            temperature: None,
            max_output_tokens: None,

            id: id,
            weight: -1,
        }
    }

    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn with_provider(mut self, provider: &str) -> Self {
        self.provider = Some(provider.to_string());
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_max_output_tokens(mut self, max_output_tokens: usize) -> Self {
        self.max_output_tokens = Some(max_output_tokens);
        self
    }

    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

// ------------------ Main Config ------------------
#[derive(Debug, Clone)]
pub struct Config {
    // global configs for models, will be overridden by model-specific configs
    pub(crate) base_url: Option<String>,
    pub(crate) provider: ProviderName,   // "AMRS" by default
    pub(crate) temperature: f32,         // 0.8 by default
    pub(crate) max_output_tokens: usize, // 1024 by default

    pub(crate) routing_mode: RoutingMode, // Random by default
    pub(crate) models: Vec<ModelConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: None,
            provider: AMRS_PROVIDER.to_string(),
            temperature: 0.8,
            max_output_tokens: 1024,
            routing_mode: RoutingMode::Random,
            models: vec![],
        }
    }
}

impl Config {
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn with_provider(mut self, provider: &str) -> Self {
        self.provider = provider.to_string();
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn with_max_output_tokens(mut self, max_output_tokens: usize) -> Self {
        self.max_output_tokens = max_output_tokens;
        self
    }

    pub fn with_routing_mode(mut self, mode: RoutingMode) -> Self {
        self.routing_mode = mode;
        self
    }

    pub fn with_model(mut self, model: ModelConfig) -> Self {
        self.models.push(model);
        self
    }

    pub fn finalize(&mut self) -> Result<(), String> {
        self.post_defaults();
        self.validate_model_config()?;
        Ok(())
    }

    fn post_defaults(&mut self) {
        for model in &mut self.models {
            let model_url_exist = model.base_url.is_some();

            if model.provider.is_none() {
                model.provider = Some(self.provider.clone());
            }

            if !model_url_exist
                && PROVIDER_BASE_URLS.contains_key(model.provider.as_ref().unwrap().as_str())
            {
                model.base_url =
                    Some(PROVIDER_BASE_URLS[model.provider.as_ref().unwrap().as_str()].to_string());
            }
            if !model_url_exist && self.base_url.is_some() {
                model.base_url = self.base_url.clone();
            }
            if model.temperature.is_none() {
                model.temperature = Some(self.temperature);
            }
            if model.max_output_tokens.is_none() {
                model.max_output_tokens = Some(self.max_output_tokens);
            }
        }
    }

    fn validate_model_config(&self) -> Result<(), String> {
        if self.models.is_empty() {
            return Err("At least one model must be configured.".to_string());
        }

        for model in &self.models {
            if model.base_url.is_none() && self.base_url.is_none() {
                return Err(format!("Model '{}' base_url is not provided.", model.id));
            }

            if self.routing_mode == RoutingMode::Weighted && model.weight <= 0 {
                return Err(format!(
                    "Model '{}' weight must be non-negative in Weighted routing mode.",
                    model.id
                ));
            }

            if let Some(max_output_tokens) = model.max_output_tokens {
                if max_output_tokens <= 0 {
                    return Err(format!(
                        "Model '{}' max_output_tokens must be positive.",
                        model.id
                    ));
                }
            }

            if let Some(temperature) = model.temperature {
                if temperature < 0.0 || temperature > 1.0 {
                    return Err(format!(
                        "Model '{}' temperature must be between 0.0 and 1.0.",
                        model.id
                    ));
                }
            }

            if let Some(provider) = &model.provider {
                let env_var = format!("{}_API_KEY", provider.to_uppercase());
                if env::var(&env_var).is_err() {
                    return Err(format!(
                        "API key for provider '{}' not found in environment variable '{}'",
                        provider.to_uppercase(),
                        env_var
                    ));
                }
            }
        }

        Ok(())
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::from_filename;

    #[test]
    fn test_config_validation() {
        from_filename(".env.test").ok();

        // case 1:
        let mut valid_simplest_models_cfg = Config::default()
            .with_provider("OPENAI")
            .with_model(ModelConfig::new("gpt-4".to_string()));
        let res = valid_simplest_models_cfg.finalize();
        assert!(res.clone().is_ok());
        assert!(
            valid_simplest_models_cfg.models[0].base_url
                == Some("https://api.openai.com/v1".to_string())
        );
        assert!(valid_simplest_models_cfg.models[0].provider == Some("OPENAI".to_string()));
        assert!(valid_simplest_models_cfg.models[0].temperature == Some(0.8));
        assert!(valid_simplest_models_cfg.models[0].max_output_tokens == Some(1024));
        assert!(valid_simplest_models_cfg.models[0].weight == -1);

        // case 2:
        let mut valid_cfg = Config::default()
            .with_provider("OPENAI")
            .with_model(ModelConfig::new("gpt-3.5-turbo".to_string()))
            .with_model(ModelConfig::new("gpt-4".to_string()));
        assert!(valid_cfg.finalize().is_ok());

        // case 3:
        let mut invalid_cfg_with_no_api_key = Config::default()
            .with_provider("unknown_provider")
            .with_model(ModelConfig::new("some-model".to_string()));
        assert!(invalid_cfg_with_no_api_key.finalize().is_err());

        // case 4:
        let mut valid_cfg_with_customized_provider = Config::default()
            .with_base_url("http://example.ai")
            .with_max_output_tokens(2048)
            .with_model(ModelConfig::new("custom-model".to_string()).with_provider("FOO"));
        let res = valid_cfg_with_customized_provider.finalize();
        assert!(res.is_ok());
        assert_eq!(
            valid_cfg_with_customized_provider.models[0]
                .base_url
                .as_ref()
                .unwrap(),
            "http://example.ai"
        );
        assert_eq!(
            valid_cfg_with_customized_provider.models[0].provider,
            Some("FOO".to_string())
        );
        assert_eq!(
            valid_cfg_with_customized_provider.models[0].max_output_tokens,
            Some(2048)
        );

        // case 5:
        let mut invalid_empty_models_cfg = Config::default().with_provider("OPENAI");
        assert!(invalid_empty_models_cfg.finalize().is_err());
    }
}
