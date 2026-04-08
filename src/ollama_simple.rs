// Simple Ollama integration for existing CalcLaw
use serde::{Deserialize, Serialize};
use std::time::Duration;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiRequest {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiResponse {
    pub response: String,
}

pub struct OllamaSimple {
    base_url: String,
}

impl OllamaSimple {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    // Check if Ollama is running
    pub async fn is_available(&self) -> bool {
        let client = reqwest::Client::new();
        match client.get(format!("{}/api/tags", self.base_url))
            .timeout(Duration::from_secs(2))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }
    
    // Generate text
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String, anyhow::Error> {
        let client = reqwest::Client::new();
        
        let request = OllamaApiRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: Some(false),
        };
        
        let response = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let api_response: OllamaApiResponse = response.json().await?;
            Ok(api_response.response)
        } else {
            Err(anyhow::anyhow!("Ollama API error: {}", response.status()))
        }
    }
}