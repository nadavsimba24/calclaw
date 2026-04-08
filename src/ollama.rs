use serde::{Deserialize, Serialize};
use reqwest;
use tracing::{info, error};
use std::time::Duration;

// Ollama API structures
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaOptions {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub num_predict: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<i32>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<i32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<i32>,
    pub eval_duration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaListResponse {
    pub models: Vec<OllamaModel>,
}

// Ollama client
pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: Option<String>) -> Self {
        let url = base_url.unwrap_or_else(|| "http://localhost:11434".to_string());
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build HTTP client");
        
        Self {
            base_url: url,
            client,
        }
    }

    // Generate text using a model
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<OllamaResponse, String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(0.7),
                top_p: Some(0.9),
                top_k: Some(40),
                num_predict: Some(512),
            }),
        };

        info!("Sending request to Ollama: model={}, prompt_length={}", model, prompt.len());
        
        match self.client.post(&url)
            .json(&request)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<OllamaResponse>().await {
                            Ok(data) => {
                                info!("Ollama response received: {} chars", data.response.len());
                                Ok(data)
                            },
                            Err(e) => {
                                error!("Failed to parse Ollama response: {}", e);
                                Err(format!("Failed to parse response: {}", e))
                            }
                        }
                    } else {
                        let status = response.status();
                        let text = response.text().await.unwrap_or_else(|_| "No error details".to_string());
                        error!("Ollama API error: {} - {}", status, text);
                        Err(format!("Ollama API error: {} - {}", status, text))
                    }
                },
                Err(e) => {
                    error!("Failed to connect to Ollama: {}", e);
                    Err(format!("Failed to connect to Ollama: {}", e))
                }
            }
    }

    // List available models
    pub async fn list_models(&self) -> Result<Vec<OllamaModel>, String> {
        let url = format!("{}/api/tags", self.base_url);
        
        info!("Listing Ollama models from {}", url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<OllamaListResponse>().await {
                        Ok(data) => {
                            info!("Found {} models", data.models.len());
                            Ok(data.models)
                        },
                        Err(e) => {
                            error!("Failed to parse models list: {}", e);
                            Err(format!("Failed to parse models list: {}", e))
                        }
                    }
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_else(|_| "No error details".to_string());
                    error!("Failed to list models: {} - {}", status, text);
                    Err(format!("Failed to list models: {} - {}", status, text))
                }
            },
            Err(e) => {
                error!("Failed to connect to Ollama for model list: {}", e);
                Err(format!("Failed to connect to Ollama: {}", e))
            }
        }
    }

    // Check if Ollama is running
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    // Generate Hebrew text with RTL support
    pub async fn generate_hebrew(&self, model: &str, prompt: &str) -> Result<String, String> {
        let hebrew_prompt = format!("{} תשובה בעברית בבקשה.", prompt);
        
        match self.generate(model, &hebrew_prompt).await {
            Ok(response) => {
                // Ensure RTL for Hebrew text
                let processed = if response.response.chars().any(|c| ('\u{0590}'..='\u{05FF}').contains(&c)) {
                    format!("\u{202B}{}\u{202C}", response.response)
                } else {
                    response.response
                };
                Ok(processed)
            },
            Err(e) => Err(e),
        }
    }

    // Generate code with a specific model
    pub async fn generate_code(&self, model: &str, language: &str, task: &str) -> Result<String, String> {
        let code_prompt = format!("Write {} code for: {}", language, task);
        
        match self.generate(model, &code_prompt).await {
            Ok(response) => Ok(response.response),
            Err(e) => Err(e),
        }
    }
}

// API request/response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub language: Option<String>,
    pub task_type: Option<String>, // "hebrew", "code", "general"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub success: bool,
    pub model: String,
    pub response: String,
    pub error: Option<String>,
    pub processing_time_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub success: bool,
    pub models: Vec<OllamaModel>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub ollama_running: bool,
    pub models_count: usize,
    pub default_models: Vec<String>,
}