use std::fmt::Display;

use serde::Deserialize;

#[derive(thiserror::Error)]
pub enum OpenAiError {
    #[error("{0}")]
    HttpError(OpenAiErrorResponse),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

impl From<reqwest::Error> for OpenAiError {
    fn from(e: reqwest::Error) -> Self {
        OpenAiError::InternalError(e.into())
    }
}

impl From<OpenAiErrorResponse> for OpenAiError {
    fn from(r: OpenAiErrorResponse) -> Self {
        OpenAiError::HttpError(r)
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAiErrorResponse {
    pub error: ErrorResponseFields,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponseFields {
    pub message: String,
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl Display for OpenAiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.error.type_, self.error.message)
    }
}

impl std::fmt::Debug for OpenAiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
