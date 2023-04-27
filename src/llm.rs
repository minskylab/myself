use reqwest::{header::HeaderMap, Client};
use serde_json::{from_str, json};
use thiserror::Error;

use crate::llm_responses::CompletionResponse;

static OPENAI_COMPLETION_API: &str = "https://api.openai.com/v1/completions";

#[derive(Error, Debug)]
pub enum LLMEngineError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Debug, Clone)]
pub struct LLMConfiguration {
    pub model_name: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: Option<f32>,
}

impl Default for LLMConfiguration {
    fn default() -> Self {
        let model_name =
            std::env::var("OPENAI_MODEL_NAME").unwrap_or("text-davinci-003".to_string());

        let max_tokens = std::env::var("OPENAI_MAX_TOKENS")
            .unwrap_or("1000".to_string())
            .parse::<usize>()
            .unwrap();

        let temperature = std::env::var("OPENAI_TEMPERATURE")
            .unwrap_or("0.75".to_string())
            .parse::<f32>()
            .unwrap();

        Self {
            model_name,
            max_tokens,
            temperature,
            top_p: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LLMEngine {
    access_token: String,
    http_client: Client,
    configuration: LLMConfiguration,
}

impl LLMEngine {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            // TODO: Make this more configurable
            configuration: LLMConfiguration::default(),
        }
    }

    pub fn new_defaults() -> Self {
        Self {
            access_token: std::env::var("OPENAI_API_KEY").unwrap(),
            http_client: Client::new(),
            configuration: LLMConfiguration::default(),
        }
    }

    pub async fn completions_call(
        &self,
        prompt: impl Into<String>,
        stop_words: Option<Vec<String>>,
    ) -> Result<CompletionResponse, LLMEngineError> {
        let endpoint = String::from(OPENAI_COMPLETION_API);

        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {}", self.access_token).parse().unwrap(),
        );

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .http_client
            .post(&endpoint)
            .headers(headers)
            .json(&json! {
                {
                    "model": self.configuration.model_name,
                    "prompt": prompt.into(),
                    "max_tokens": self.configuration.max_tokens,
                    "temperature": self.configuration.temperature,
                    "stop": stop_words,
                    "top_p": self.configuration.top_p.unwrap_or(1.0),
                    // "n": 1,
                    // "stream": false,
                    // "logprobs": null,
                    // "stop": "\n"
                }
            })
            .send()
            .await?;
        // .unwrap_or_else(op);

        let response_text = response.text().await.unwrap();

        let data = from_str::<CompletionResponse>(&response_text).unwrap();

        Ok(data)
    }
}
