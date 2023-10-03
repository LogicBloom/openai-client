use serde::{Deserialize, Serialize};

use crate::{
    error::{OpenAiError, OpenAiErrorResponse},
    Client as OpenAiClient,
};

#[derive(Debug, Serialize)]
pub struct ChatCompletionsPayload {
    model: String,
    temperature: f32,
    messages: Vec<CompletionMessage>,
}

#[derive(Debug, Clone)]
pub struct ChatCompletionsBuilder<'a> {
    client: &'a OpenAiClient,
    model: String,
    temperature: f32,
    messages: Vec<CompletionMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionMessage {
    role: CompletionRoles,
    content: String,
}

impl CompletionMessage {
    pub fn new<S: Into<String>>(role: CompletionRoles, content: S) -> Self {
        CompletionMessage {
            role,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompletionRoles {
    Assistant,
    System,
    User,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionsSuccessResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: CompletionMessage,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl<'a> ChatCompletionsBuilder<'a> {
    #[must_use]
    pub fn new<S: Into<String>>(client: &'a OpenAiClient, model: S) -> Self {
        ChatCompletionsBuilder {
            client,
            model: model.into(),
            messages: Vec::new(),
            temperature: 0f32,
        }
    }

    pub fn temperature<N: Into<f32>>(mut self, value: N) -> ChatCompletionsBuilder<'a> {
        self.temperature = value.into();
        self
    }

    pub fn message(mut self, value: CompletionMessage) -> ChatCompletionsBuilder<'a> {
        self.messages.push(value);
        self
    }

    pub async fn send(self) -> Result<ChatCompletionsSuccessResponse, OpenAiError> {
        let request_body = ChatCompletionsPayload {
            messages: self.messages,
            model: self.model,
            temperature: self.temperature,
        };
        let response = self
            .client
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.client.api_key)
            .json(&request_body)
            .send()
            .await?;
        if !response.status().is_success() {
            let response = response.json::<OpenAiErrorResponse>().await?;
            return Err(response.into());
        }
        let response = response.json::<ChatCompletionsSuccessResponse>().await?;
        Ok(response)
    }
}
