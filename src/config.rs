use std::collections::HashMap;
use std::env;

use derive_builder::Builder;
use lazy_static::lazy_static;

// ------------------ Provider ------------------
pub type ProviderName = String;
const OPENAI_PROVIDER: &str = "OPENAI";

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
    WRR, // WeightedRoundRobin,
}

// ------------------ Model Config ------------------
pub type ModelId = String;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"), pattern = "mutable")]
pub struct ModelConfig {
    // model-specific configs, will override global configs if provided
    #[builder(default = "None")]
    pub base_url: Option<String>,
    #[builder(default = "None")]
    pub provider: Option<ProviderName>,
    #[builder(default = "None")]
    pub temperature: Option<f32>,
    #[builder(default = "None")]
    pub max_output_tokens: Option<usize>,

    pub id: ModelId,
    #[builder(default=-1)]
    pub weight: i32,
}

impl ModelConfigBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() {
            return Err("Model id must be provided.".to_string());
        }
        Ok(())
    }
}

impl ModelConfig {
    pub fn builder() -> ModelConfigBuilder {
        ModelConfigBuilder::default()
    }
}

// ------------------ Main Config ------------------
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"), pattern = "mutable")]
pub struct Config {
    // global configs for models, will be overridden by model-specific configs
    #[builder(default = "https://api.openai.com/v1".to_string())]
    pub(crate) base_url: String,
    #[builder(default = "ProviderName::from(OPENAI_PROVIDER)")]
    pub(crate) provider: ProviderName,
    #[builder(default = "0.8")]
    pub(crate) temperature: f32,
    #[builder(default = "1024")]
    pub(crate) max_output_tokens: usize,

    #[builder(default = "RoutingMode::Random")]
    pub(crate) routing_mode: RoutingMode,
    #[builder(default = "vec![]")]
    pub(crate) models: Vec<ModelConfig>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    // populate will fill in the missing model-specific configs with global configs.
    pub fn populate(&mut self) -> &mut Self {
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
            if !model_url_exist {
                model.base_url = Some(self.base_url.clone());
            }
            if model.temperature.is_none() {
                model.temperature = Some(self.temperature);
            }
            if model.max_output_tokens.is_none() {
                model.max_output_tokens = Some(self.max_output_tokens);
            }
        }
        self
    }
}

impl ConfigBuilder {
    pub fn model(&mut self, model: ModelConfig) -> &mut Self {
        let mut models = self.models.clone().unwrap_or_default();
        models.push(model);
        self.models = Some(models);
        self
    }

    fn validate(&self) -> Result<(), String> {
        if self.models.is_none() || self.models.as_ref().unwrap().is_empty() {
            return Err("At least one model must be configured.".to_string());
        }

        for model in self.models.as_ref().unwrap() {
            if self.routing_mode.is_some()
                && self.routing_mode.as_ref().unwrap() == &RoutingMode::WRR
                && model.weight <= 0
            {
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

            // check the existence of API key in environment variables
            if let Some(provider) = &model.provider {
                let env_var = format!("{}_API_KEY", provider.to_uppercase());
                if env::var(&env_var).is_err() {
                    return Err(format!(
                        "API key for provider '{}' not found in environment variable '{}'",
                        provider.to_uppercase(),
                        env_var
                    ));
                }
            } else {
                // default is called after validation.
                let env_var = format!(
                    "{}_API_KEY",
                    self.provider
                        .as_ref()
                        .unwrap_or(&ProviderName::from(OPENAI_PROVIDER))
                        .to_uppercase()
                );
                if env::var(&env_var).is_err() {
                    return Err(format!(
                        "API key for provider '{}' not found in environment variable '{}'",
                        self.provider
                            .as_ref()
                            .unwrap_or(&ProviderName::from(OPENAI_PROVIDER))
                            .to_uppercase(),
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
        let valid_simplest_models_cfg = Config::builder()
            .model(
                ModelConfig::builder()
                    .id("gpt-4".to_string())
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(valid_simplest_models_cfg.is_ok());
        assert!(valid_simplest_models_cfg.as_ref().unwrap().provider == OPENAI_PROVIDER);
        assert!(
            valid_simplest_models_cfg.as_ref().unwrap().base_url == "https://api.openai.com/v1"
        );
        assert!(valid_simplest_models_cfg.as_ref().unwrap().temperature == 0.8);
        assert!(
            valid_simplest_models_cfg
                .as_ref()
                .unwrap()
                .max_output_tokens
                == 1024
        );
        assert!(valid_simplest_models_cfg.as_ref().unwrap().routing_mode == RoutingMode::Random);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models.len() == 1);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].base_url == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].provider == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].temperature == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].max_output_tokens == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].weight == -1);

        // case 2:
        let valid_cfg = Config::builder()
            .models(vec![
                ModelConfig::builder()
                    .id("gpt-3.5-turbo".to_string())
                    .build()
                    .unwrap(),
                ModelConfig::builder()
                    .id("gpt-4".to_string())
                    .build()
                    .unwrap(),
            ])
            .build();
        assert!(valid_cfg.is_ok());
        assert!(valid_cfg.as_ref().unwrap().models.len() == 2);

        // case 3:
        let invalid_cfg_with_no_api_key = Config::builder()
            .model(
                ModelConfig::builder()
                    .id("some-model".to_string())
                    .build()
                    .unwrap(),
            )
            .provider("unknown_provider".to_string())
            .build();
        assert!(invalid_cfg_with_no_api_key.is_err());

        // case 4:
        // AMRS_API_KEY is set in .env.test already.
        let valid_cfg_with_customized_provider = Config::builder()
            .base_url("http://example.ai".to_string())
            .max_output_tokens(2048)
            .model(
                ModelConfig::builder()
                    .id("custom-model".to_string())
                    .provider(Some("AMRS".to_string()))
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(valid_cfg_with_customized_provider.is_ok());

        // case 5:
        let invalid_empty_models_cfg = Config::builder().build();
        assert!(invalid_empty_models_cfg.is_err());

        // case 6:
        print!("validating invalid empty model id config");
        let invalid_empty_model_id_cfg = ModelConfig::builder().build();
        assert!(invalid_empty_model_id_cfg.is_err());
    }

    #[test]
    fn test_populate_config() {
        from_filename(".env.test").ok();

        let mut valid_cfg = Config::builder()
            .temperature(0.5)
            .max_output_tokens(1500)
            .model(
                ModelConfig::builder()
                    .id("model-1".to_string())
                    .build()
                    .unwrap(),
            )
            .build();
        valid_cfg.as_mut().unwrap().populate();

        assert!(valid_cfg.is_ok());
        assert!(valid_cfg.as_ref().unwrap().models.len() == 1);
        assert!(valid_cfg.as_ref().unwrap().models[0].temperature == Some(0.5));
        assert!(valid_cfg.as_ref().unwrap().models[0].max_output_tokens == Some(1500));
        assert!(valid_cfg.as_ref().unwrap().models[0].provider == Some("OPENAI".to_string()));
        assert!(
            valid_cfg.as_ref().unwrap().models[0].base_url
                == Some("https://api.openai.com/v1".to_string())
        );
        assert!(valid_cfg.as_ref().unwrap().models[0].weight == -1);

        let mut valid_specified_cfg = Config::builder()
            .provider("AMRS".to_string())
            .base_url("http://custom-api.ai".to_string())
            .model(
                ModelConfig::builder()
                    .id("model-2".to_string())
                    .build()
                    .unwrap(),
            )
            .build();
        valid_specified_cfg.as_mut().unwrap().populate();

        assert!(valid_specified_cfg.is_ok());
        assert!(valid_specified_cfg.as_ref().unwrap().provider == "AMRS".to_string());
        assert!(
            valid_specified_cfg.as_ref().unwrap().models[0].base_url
                == Some("http://custom-api.ai".to_string())
        );
    }
}
