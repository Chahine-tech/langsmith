use std::env;

/// Configuration for API providers
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub provider: ApiProvider,
    pub api_key: String,
}

/// Supported API providers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiProvider {
    DeepL,
    OpenAI,
}

impl ApiProvider {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "deepl" => Ok(ApiProvider::DeepL),
            "openai" => Ok(ApiProvider::OpenAI),
            _ => Err(anyhow::anyhow!(
                "Unknown provider: {}. Supported: deepl, openai",
                s
            )),
        }
    }

    pub fn env_var_name(&self) -> &'static str {
        match self {
            ApiProvider::DeepL => "DEEPL_API_KEY",
            ApiProvider::OpenAI => "OPENAI_API_KEY",
        }
    }
}

/// Manages API configuration with priority order
pub struct ConfigManager;

impl ConfigManager {
    /// Get API configuration with priority:
    /// 1. CLI flag (highest)
    /// 2. Environment variable
    /// 3. Error (lowest)
    pub fn get_api_config(provider: &str, cli_api_key: Option<&str>) -> anyhow::Result<ApiConfig> {
        let provider = ApiProvider::from_str(provider)?;

        // Priority 1: CLI flag
        if let Some(key) = cli_api_key {
            if key.is_empty() {
                return Err(anyhow::anyhow!("API key provided via CLI is empty"));
            }
            tracing::debug!("Using API key from CLI flag");
            return Ok(ApiConfig {
                provider,
                api_key: key.to_string(),
            });
        }

        // Priority 2: Environment variable
        let env_var = provider.env_var_name();
        if let Ok(key) = env::var(env_var) {
            if key.is_empty() {
                return Err(anyhow::anyhow!("Environment variable {} is empty", env_var));
            }
            tracing::debug!("Using API key from environment variable: {}", env_var);
            return Ok(ApiConfig {
                provider,
                api_key: key,
            });
        }

        // Priority 3: Error
        Err(anyhow::anyhow!(
            "API key not found. Set {} environment variable or use --api-key flag",
            env_var
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_from_str() {
        assert_eq!(ApiProvider::from_str("deepl").unwrap(), ApiProvider::DeepL);
        assert_eq!(
            ApiProvider::from_str("openai").unwrap(),
            ApiProvider::OpenAI
        );
        assert_eq!(ApiProvider::from_str("DEEPL").unwrap(), ApiProvider::DeepL);
        assert!(ApiProvider::from_str("invalid").is_err());
    }

    #[test]
    fn test_config_cli_priority() {
        let config = ConfigManager::get_api_config("deepl", Some("cli-key"));
        assert!(config.is_ok());
        assert_eq!(config.unwrap().api_key, "cli-key");
    }
}
