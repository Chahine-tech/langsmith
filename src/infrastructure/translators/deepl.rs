use crate::domain::ports::Translator;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// DeepL translation API implementation
pub struct DeepLTranslator {
    api_key: String,
    client: reqwest::Client,
}

impl DeepLTranslator {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Convert language code to DeepL target language
    /// e.g., "en" -> "EN", "fr" -> "FR"
    fn normalize_target_lang(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "en" => "EN-US".to_string(),
            "fr" => "FR".to_string(),
            "es" => "ES".to_string(),
            "de" => "DE".to_string(),
            "it" => "IT".to_string(),
            "pt" => "PT-PT".to_string(),
            "nl" => "NL".to_string(),
            "pl" => "PL".to_string(),
            "ru" => "RU".to_string(),
            "ja" => "JA".to_string(),
            "zh" => "ZH".to_string(),
            other => other.to_uppercase(),
        }
    }
}

#[derive(Serialize)]
struct DeepLRequest {
    text: Vec<String>,
    target_lang: String,
}

#[derive(Deserialize)]
struct DeepLTranslation {
    text: String,
}

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<DeepLTranslation>,
}

#[async_trait]
impl Translator for DeepLTranslator {
    async fn translate(&self, text: &str, target_lang: &str) -> anyhow::Result<String> {
        let normalized_lang = self.normalize_target_lang(target_lang);

        let request = DeepLRequest {
            text: vec![text.to_string()],
            target_lang: normalized_lang.clone(),
        };

        // Debug: log key info (first and last chars only)
        let key_preview = if self.api_key.len() > 10 {
            format!("{}...{}", &self.api_key[..5], &self.api_key[self.api_key.len()-5..])
        } else {
            "***".to_string()
        };
        tracing::debug!("DeepL API key preview: {}", key_preview);
        tracing::debug!("Translating '{}' to {}", text, normalized_lang);

        let response = self
            .client
            .post("https://api-free.deepl.com/v2/translate")
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        tracing::debug!("DeepL API response status: {}", response.status());

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("DeepL API error details: {}", error_text);
            return Err(anyhow::anyhow!(
                "DeepL API error ({}): {}",
                status,
                error_text
            ));
        }

        let data: DeepLResponse = response.json().await?;

        if data.translations.is_empty() {
            return Err(anyhow::anyhow!("DeepL returned empty translations"));
        }

        Ok(data.translations[0].text.clone())
    }
}
