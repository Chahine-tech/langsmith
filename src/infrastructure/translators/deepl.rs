use crate::domain::ports::Translator;
use async_trait::async_trait;

/// DeepL translation API implementation (Phase 2)
#[allow(dead_code)]
pub struct DeepLTranslator {
    api_key: String,
}

impl DeepLTranslator {
    #[allow(dead_code)]
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Translator for DeepLTranslator {
    async fn translate(&self, _text: &str, _target_lang: &str) -> anyhow::Result<String> {
        // TODO: Implement Phase 2
        todo!("DeepL translation implementation in Phase 2")
    }
}
