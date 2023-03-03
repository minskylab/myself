use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Client};
use serde_json::{from_str, json};

use crate::llm_responses::CompletionResponse;

static OPENAI_COMPLETION_API: &str = "https://api.openai.com/v1/completions";


#[derive(Debug, Clone)]
pub struct LLMEngine {
    access_token: String,
    http_client: Client,
}

impl LLMEngine {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }

    pub fn new_with_defaults() -> Self {
        Self {
            access_token: std::env::var("OPENAI_API_KEY").unwrap(),
            http_client: Client::new(),
        }
    }

    pub async fn completions_call(
        self: Self,
        prompt: impl Into<String>,
        stop_words: Option<Vec<String>>,
    ) -> Result<CompletionResponse> {
        let endpoint = String::from(OPENAI_COMPLETION_API);
    
        let mut headers = HeaderMap::new();
    
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.access_token).parse().unwrap(),
        );
    
        headers.insert("Content-Type", "application/json".parse().unwrap());
    
        let model_name = "text-davinci-003";
        // let model_name = "text-davinci-003"; // "code-davinci-002"; // "text-davinci-003"
    
        let response = self
            .http_client
            .post(&endpoint)
            .headers(headers)
            .json(&json! {
                {
                    "model": model_name,
                    "prompt": prompt.into(),
                    "max_tokens": 1000,
                    "temperature": 0.7,
                    "stop": stop_words,
                    // "top_p": 1,
                    // "n": 1,
                    // "stream": false,
                    // "logprobs": null,
                    // "stop": "\n"
                }
            })
            .send()
            .await?;
    
        let response_text = response.text().await.unwrap();
    
        let Ok(data) = from_str::<CompletionResponse>(&response_text) else {
            // let response_text = response_text;
            return Err(anyhow!(response_text));
    
        };
    
        Ok(data)
    }
    
}
