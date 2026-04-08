// Simple Ollama integration for existing CalcLaw
use serde::{Deserialize, Serialize};
use std::time::Duration;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaGenerateResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaListResponse {
    pub models: Vec<OllamaModel>,
}

pub struct OllamaSimple {
    base_url: String,
}

impl OllamaSimple {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
        }
    }
    
    pub fn with_url(url: String) -> Self {
        Self { base_url: url }
    }
    
    // Check if Ollama is running
    pub async fn is_available(&self) -> bool {
        let client = reqwest::Client::new();
        match client.get(format!("{}/api/tags", self.base_url))
            .timeout(Duration::from_secs(2))
            .send()
            .await {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            }
    }
    
    // Get available models
    pub async fn get_models(&self) -> Result<Vec<OllamaModel>, String> {
        let client = reqwest::Client::new();
        match client.get(format!("{}/api/tags", self.base_url))
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<OllamaListResponse>().await {
                            Ok(data) => Ok(data.models),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        Err(format!("HTTP error: {}", response.status()))
                    }
                },
                Err(e) => Err(format!("Request failed: {}", e)),
            }
    }
    
    // Generate text
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let request = OllamaGenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };
        
        match client.post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<OllamaGenerateResponse>().await {
                            Ok(data) => Ok(data.response),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        Err(format!("HTTP error: {}", response.status()))
                    }
                },
                Err(e) => Err(format!("Request failed: {}", e)),
            }
    }
    
    // Generate Hebrew text with RTL
    pub async fn generate_hebrew(&self, model: &str, prompt: &str) -> Result<String, String> {
        let hebrew_prompt = format!("{} תשובה בעברית בבקשה.", prompt);
        self.generate(model, &hebrew_prompt).await
    }
}

// API request/response structures for integration
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiRequest {
    pub model: String,
    pub prompt: String,
    pub task_type: Option<String>, // "hebrew", "general"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiResponse {
    pub success: bool,
    pub model: String,
    pub response: String,
    pub error: Option<String>,
}