use crate::domain::ports::Translator;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// OpenAI translation API implementation
pub struct OpenAITranslator {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAITranslator {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Convert language code to full language name
    /// e.g., "en" -> "English", "fr" -> "French"
    fn lang_code_to_name(&self, code: &str) -> String {
        match code.to_lowercase().as_str() {
            "en" => "English".to_string(),
            "fr" => "French".to_string(),
            "es" => "Spanish".to_string(),
            "de" => "German".to_string(),
            "it" => "Italian".to_string(),
            "pt" => "Portuguese".to_string(),
            "nl" => "Dutch".to_string(),
            "pl" => "Polish".to_string(),
            "ru" => "Russian".to_string(),
            "ja" => "Japanese".to_string(),
            "zh" => "Chinese".to_string(),
            "ko" => "Korean".to_string(),
            other => other.to_uppercase(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[async_trait]
impl Translator for OpenAITranslator {
    async fn translate(&self, text: &str, target_lang: &str) -> anyhow::Result<String> {
        let target_lang_name = self.lang_code_to_name(target_lang);

        let system_prompt = format!(
            "You are a professional translator. Translate the following text to {}. \
             Return only the translated text, no explanations, no markdown formatting.",
            target_lang_name
        );

        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
            temperature: 0.3,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "OpenAI API error ({}): {}",
                status,
                error_text
            ));
        }

        let data: OpenAIResponse = response.json().await?;

        if data.choices.is_empty() {
            return Err(anyhow::anyhow!("OpenAI returned no choices"));
        }

        let translated = data.choices[0].message.content.trim().to_string();
        Ok(translated)
    }
}
