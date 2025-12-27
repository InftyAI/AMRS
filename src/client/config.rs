use std::collections::HashMap;
use std::env;

use derive_builder::Builder;
use lazy_static::lazy_static;

// ------------------ Provider ------------------
pub const DEFAULT_PROVIDER: &str = "OPENAI";

lazy_static! {
    pub static ref PROVIDER_BASE_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("OPENAI", "https://api.openai.com/v1");
        m.insert("DEEPINFRA", "https://api.deepinfra.com/v1/openai");
        m.insert("OPENROUTER", "https://openrouter.ai/api/v1");

        m.insert("FAKER", "http://localhost:8080"); // test only
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
pub type ModelName = String;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"), pattern = "mutable")]
pub struct ModelConfig {
    // model-specific configs, will override global configs if provided
    #[builder(default = "None")]
    pub(crate) base_url: Option<String>,
    #[builder(default = "None", setter(custom))]
    pub(crate) provider: Option<String>,

    #[builder(setter(custom))]
    pub(crate) name: ModelName,
    #[builder(default=-1)]
    pub(crate) weight: i32,
}

impl ModelConfigBuilder {
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    pub fn provider<S>(&mut self, name: Option<S>) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.provider = Some(name.map(|s| s.as_ref().to_string().to_uppercase()));
        self
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_none() {
            return Err("Model name must be provided.".to_string());
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
    #[builder(default=None, setter(custom))]
    pub(crate) base_url: Option<String>,
    #[builder(default = "DEFAULT_PROVIDER.to_string()", setter(custom))]
    pub(crate) provider: String,

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
        let global_base_url = match self.base_url.is_some() {
            true => self.base_url.clone(),
            false => Some(
                PROVIDER_BASE_URLS
                    .get(self.provider.as_str())
                    .unwrap()
                    .to_string(),
            ),
        };

        for model in &mut self.models {
            if model.base_url.is_none() {
                if model.provider.is_some() {
                    model.base_url = Some(
                        PROVIDER_BASE_URLS
                            .get(model.provider.as_ref().unwrap().as_str())
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    model.base_url = global_base_url.clone();
                }
            }

            if model.provider.is_none() {
                model.provider = Some(self.provider.clone());
            }
        }
        self
    }
}

impl ConfigBuilder {
    pub fn base_url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
        self.base_url = Some(Some(url.as_ref().to_string()));
        self
    }
    pub fn model(&mut self, model: ModelConfig) -> &mut Self {
        let mut models = self.models.clone().unwrap_or_default();
        models.push(model);
        self.models = Some(models);
        self
    }

    pub fn provider<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.provider = Some(name.as_ref().to_string().to_uppercase());
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
                    model.name
                ));
            }

            // check the existence of API key in environment variables
            if let Some(provider) = &model.provider {
                let env_var = format!("{}_API_KEY", provider);
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
                        .unwrap_or(&DEFAULT_PROVIDER.to_string())
                        .to_uppercase()
                );
                if env::var(&env_var).is_err() {
                    return Err(format!(
                        "API key for provider '{}' not found in environment variable '{}'",
                        self.provider
                            .as_ref()
                            .unwrap_or(&DEFAULT_PROVIDER.to_string()),
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
                    .name("gpt-4".to_string())
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(valid_simplest_models_cfg.is_ok());
        assert!(valid_simplest_models_cfg.as_ref().unwrap().provider == DEFAULT_PROVIDER);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().base_url == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().routing_mode == RoutingMode::Random);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models.len() == 1);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].base_url == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].provider == None);
        assert!(valid_simplest_models_cfg.as_ref().unwrap().models[0].weight == -1);

        // case 2:
        let valid_cfg = Config::builder()
            .models(vec![
                ModelConfig::builder()
                    .name("gpt-3.5-turbo".to_string())
                    .build()
                    .unwrap(),
                ModelConfig::builder()
                    .name("gpt-4".to_string())
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
                    .name("some-model".to_string())
                    .build()
                    .unwrap(),
            )
            .provider("unknown_provider")
            .build();
        assert!(invalid_cfg_with_no_api_key.is_err());

        // case 4:
        // AMRS_API_KEY is set in .env.test already.
        let valid_cfg_with_customized_provider = Config::builder()
            .base_url("http://example.ai")
            .model(
                ModelConfig::builder()
                    .name("custom-model")
                    .provider(Some("AMRS"))
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(valid_cfg_with_customized_provider.is_ok());

        // case 5:
        let invalid_empty_models_cfg = Config::builder().build();
        assert!(invalid_empty_models_cfg.is_err());

        // case 6:
        print!("validating invalid empty model name config");
        let invalid_empty_model_id_cfg = ModelConfig::builder().build();
        assert!(invalid_empty_model_id_cfg.is_err());
    }

    #[test]
    fn test_populate_config() {
        from_filename(".env.test").ok();

        let mut valid_cfg = Config::builder()
            .model(
                ModelConfig::builder()
                    .name("model-1".to_string())
                    .build()
                    .unwrap(),
            )
            .build();
        valid_cfg.as_mut().unwrap().populate();

        assert!(valid_cfg.is_ok());
        assert!(valid_cfg.as_ref().unwrap().models.len() == 1);
        assert!(valid_cfg.as_ref().unwrap().models[0].provider == Some("OPENAI".to_string()));
        assert!(
            valid_cfg.as_ref().unwrap().models[0].base_url
                == Some("https://api.openai.com/v1".to_string())
        );
        assert!(valid_cfg.as_ref().unwrap().models[0].weight == -1);

        let mut valid_specified_cfg = Config::builder()
            .provider("AMRS".to_string())
            .base_url("http://custom-api.ai".to_string())
            .model(ModelConfig::builder().name("model-2").build().unwrap())
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
