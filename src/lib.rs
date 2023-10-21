mod chat_completions;
pub mod error;

use anyhow::Context;
use chat_completions::ChatCompletionsBuilder;
pub use chat_completions::{CompletionMessage, CompletionRoles};
use error::OpenAiError;
use reqwest::Client as HttpClient;
use secrecy::Secret;

/// Get current package version from metadata
const CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    api_key: Secret<String>,
}

impl<'a> Client {
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self, OpenAiError> {
        let http_client = HttpClient::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .user_agent(format!(
                "chatterverse-openai-client@{CARGO_PACKAGE_VERSION}"
            ))
            // XXX: temp fix for address unable to get local issuer certificate error
            .danger_accept_invalid_certs(true)
            .build()
            .context("Failed to build http client")?;
        Ok(Self {
            http_client,
            api_key: Secret::new(api_key.into()),
        })
    }

    pub fn chat_completions(&'a self, model: &str) -> ChatCompletionsBuilder<'a> {
        ChatCompletionsBuilder::new(self, model)
    }
}
