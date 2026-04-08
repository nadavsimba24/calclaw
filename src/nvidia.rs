use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

// NVIDIA NIM API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for NvidiaConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://integrate.api.nvidia.com/v1".to_string(),
            model: "meta/llama-3.1-8b-instruct".to_string(),
            temperature: 0.7,
            max_tokens: 1024,
        }
    }
}

// NVIDIA API request/response structures
#[derive(Debug, Serialize)]
struct NvidiaRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct NvidiaResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    total_tokens: u32,
    completion_tokens: u32,
}

// Hebrew-aware prompt formatting
fn format_hebrew_prompt(user_input: &str, is_hebrew: bool) -> String {
    if is_hebrew {
        format!(
            "אתה CalcLaw, עוזר AI דובר עברית לארגונים. \
            ענה בעברית עם תמיכה מלאה בימין-לשמאל. \
            שאלת המשתמש: {}\n\nתשובה:",
            user_input
        )
    } else {
        format!(
            "You are CalcLaw, an AI assistant for organizations with Hebrew support. \
            User question: {}\n\nAnswer:",
            user_input
        )
    }
}

// Main NVIDIA API client
pub struct NvidiaClient {
    config: Arc<RwLock<NvidiaConfig>>,
    client: Client,
}

impl NvidiaClient {
    pub fn new(config: NvidiaConfig) -> Result<Self> {
        // Create HTTP client with proper headers
        let client = Client::builder()
            .user_agent("CalcLaw/0.1.0")
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            client,
        })
    }

    pub async fn update_config(&self, new_config: NvidiaConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    pub async fn get_config(&self) -> NvidiaConfig {
        let config = self.config.read().await;
        config.clone()
    }

    // Check if text contains Hebrew
    fn is_hebrew(&self, text: &str) -> bool {
        text.chars().any(|c| ('\u{0590}'..='\u{05FF}').contains(&c))
    }

    // Generate completion using NVIDIA NIM API
    pub async fn generate(
        &self,
        prompt: &str,
        conversation_history: Option<Vec<Message>>,
    ) -> Result<String> {
        let config = self.config.read().await;
        
        if config.api_key.is_empty() {
            return Err(anyhow!("NVIDIA API key not configured"));
        }

        let is_hebrew = self.is_hebrew(prompt);
        let formatted_prompt = format_hebrew_prompt(prompt, is_hebrew);

        // Build messages
        let mut messages = Vec::new();
        
        // Add system message
        messages.push(Message {
            role: "system".to_string(),
            content: if is_hebrew {
                "אתה CalcLaw, עוזר AI דובר עברית לארגונים. ענה בעברית עם תמיכה מלאה בימין-לשמאל. השתמש בתשובות ברורות ומועילות."
                    .to_string()
            } else {
                "You are CalcLaw, an AI assistant for organizations with Hebrew support. Provide clear, helpful responses."
                    .to_string()
            },
        });

        // Add conversation history if provided
        if let Some(history) = conversation_history {
            messages.extend(history);
        }

        // Add user message
        messages.push(Message {
            role: "user".to_string(),
            content: formatted_prompt,
        });

        // Build request
        let request = NvidiaRequest {
            model: config.model.clone(),
            messages,
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            stream: false,
        };

        let url = format!("{}/chat/completions", config.base_url);

        debug!("Sending request to NVIDIA API: {}", url);
        debug!("Model: {}, Tokens: {}", config.model, config.max_tokens);

        // Send request
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request to NVIDIA API: {}", e))?;

        // Check response status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("NVIDIA API error {}: {}", status, error_text);
            return Err(anyhow!("NVIDIA API error {}: {}", status, error_text));
        }

        // Parse response
        let api_response: NvidiaResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse NVIDIA API response: {}", e))?;

        // Extract the response text
        let answer = api_response
            .choices
            .first()
            .ok_or_else(|| anyhow!("No choices in response"))?
            .message
            .content
            .clone();

        info!(
            "NVIDIA API response: {} tokens used (prompt: {}, completion: {})",
            api_response.usage.total_tokens,
            api_response.usage.prompt_tokens,
            api_response.usage.completion_tokens
        );

        Ok(answer)
    }

    // Test API connection
    pub async fn test_connection(&self) -> Result<String> {
        let config = self.config.read().await;
        
        if config.api_key.is_empty() {
            return Err(anyhow!("NVIDIA API key not configured"));
        }

        let test_prompt = "Hello, are you working?";
        let test_messages = vec![Message {
            role: "user".to_string(),
            content: test_prompt.to_string(),
        }];

        let url = format!("{}/chat/completions", config.base_url);
        
        let request = NvidiaRequest {
            model: config.model.clone(),
            messages: test_messages,
            temperature: 0.1,
            max_tokens: 10,
            stream: false,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            Ok("NVIDIA API connection successful".to_string())
        } else {
            let status = response.status();
            let error = response.text().await.unwrap_or_default();
            Err(anyhow!("Connection test failed: {} - {}", status, error))
        }
    }

    // List available models (if API supports it)
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let config = self.config.read().await;
        
        if config.api_key.is_empty() {
            return Err(anyhow!("NVIDIA API key not configured"));
        }

        let url = format!("{}/models", config.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch models: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch models: {}", response.status()));
        }

        // Try to parse as OpenAI-compatible models response
        #[derive(Debug, Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelInfo>,
        }

        #[derive(Debug, Deserialize)]
        struct ModelInfo {
            id: String,
        }

        let models_response: ModelsResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse models response: {}", e))?;

        let models = models_response
            .data
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}

// Common NVIDIA NIM models
pub const NVIDIA_MODELS: &[(&str, &str)] = &[
    ("meta/llama-3.1-8b-instruct", "Llama 3.1 8B Instruct"),
    ("meta/llama-3.1-70b-instruct", "Llama 3.1 70B Instruct"),
    ("meta/llama-3.2-1b-instruct", "Llama 3.2 1B Instruct"),
    ("meta/llama-3.2-3b-instruct", "Llama 3.2 3B Instruct"),
    ("mistralai/mistral-7b-instruct", "Mistral 7B Instruct"),
    ("mistralai/mixtral-8x7b-instruct", "Mixtral 8x7B Instruct"),
    ("google/gemma-2-2b-it", "Gemma 2 2B Instruct"),
    ("google/gemma-2-9b-it", "Gemma 2 9B Instruct"),
    ("google/gemma-2-27b-it", "Gemma 2 27B Instruct"),
    ("microsoft/phi-3-mini-128k-instruct", "Phi-3 Mini 128K Instruct"),
    ("nvidia/llama-3.1-nemotron-70b-instruct", "Nemotron 70B Instruct"),
];

// Helper to get model display name
pub fn get_model_display_name(model_id: &str) -> &str {
    NVIDIA_MODELS
        .iter()
        .find(|(id, _)| *id == model_id)
        .map(|(_, name)| *name)
        .unwrap_or(model_id)
}