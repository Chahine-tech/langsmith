/// Translation API implementations
pub mod deepl;
pub mod openai;

pub use deepl::DeepLTranslator;
pub use openai::OpenAITranslator;
