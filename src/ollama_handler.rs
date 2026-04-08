use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

use crate::ollama::{OllamaClient, GenerateRequest, GenerateResponse, ModelsResponse, HealthResponse};

// App state with Ollama client
pub struct OllamaAppState {
    pub ollama_client: OllamaClient,
    pub default_model: String,
}

// API handlers for Ollama integration
pub async fn ollama_health(
    State(state): State<Arc<OllamaAppState>>,
) -> impl IntoResponse {
    let ollama_running = state.ollama_client.health_check().await;
    
    let models = if ollama_running {
        match state.ollama_client.list_models().await {
            Ok(models) => models.len(),
            Err(_) => 0,
        }
    } else {
        0
    };

    let default_models = vec![
        "phi3:mini".to_string(),
        "gemma2:9b".to_string(),
        "phi3:3.8b".to_string(),
    ];

    let response = HealthResponse {
        ollama_running,
        models_count: models,
        default_models,
    };

    (StatusCode::OK, Json(response))
}

pub async fn list_models(
    State(state): State<Arc<OllamaAppState>>,
) -> impl IntoResponse {
    match state.ollama_client.list_models().await {
        Ok(models) => {
            let response = ModelsResponse {
                success: true,
                models,
                error: None,
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ModelsResponse {
                success: false,
                models: vec![],
                error: Some(e),
            };
            (StatusCode::SERVICE_UNAVAILABLE, Json(response))
        }
    }
}

pub async fn generate_text(
    State(state): State<Arc<OllamaAppState>>,
    Json(payload): Json<GenerateRequest>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    
    info!("Generating text with model: {}", payload.model);
    
    let result = match payload.task_type.as_deref() {
        Some("hebrew") => {
            state.ollama_client.generate_hebrew(&payload.model, &payload.prompt).await
        },
        Some("code") => {
            let language = payload.language.unwrap_or_else(|| "Python".to_string());
            state.ollama_client.generate_code(&payload.model, &language, &payload.prompt).await
        },
        _ => {
            state.ollama_client.generate(&payload.model, &payload.prompt).await
                .map(|r| r.response)
        }
    };

    let processing_time = start_time.elapsed().as_millis() as u64;

    match result {
        Ok(response) => {
            let api_response = GenerateResponse {
                success: true,
                model: payload.model,
                response,
                error: None,
                processing_time_ms: Some(processing_time),
            };
            info!("Generation successful: {} ms", processing_time);
            (StatusCode::OK, Json(api_response))
        },
        Err(e) => {
            let api_response = GenerateResponse {
                success: false,
                model: payload.model,
                response: String::new(),
                error: Some(e),
                processing_time_ms: Some(processing_time),
            };
            error!("Generation failed: {}", api_response.error.as_ref().unwrap());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response))
        }
    }
}

pub async fn ollama_dashboard() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html dir="rtl" lang="he">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>CalcLaw - Ollama Integration</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .hebrew { text-align: right; direction: rtl; }
                .header { background: #2196F3; color: white; padding: 20px; border-radius: 5px; }
                .status { background: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .feature { background: #e3f2fd; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .api-section { background: #fff3e0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .endpoint { font-family: monospace; background: #f5f5f5; padding: 5px; border-radius: 3px; }
                .form-group { margin: 10px 0; }
                label { display: block; margin-bottom: 5px; font-weight: bold; }
                input, select, textarea { width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }
                button { background: #4CAF50; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; }
                button:hover { background: #45a049; }
                .response { background: #f9f9f9; padding: 15px; border-left: 4px solid #4CAF50; margin: 10px 0; }
                .error { border-left-color: #f44336; }
            </style>
            <script>
                async function checkHealth() {
                    const response = await fetch('/api/ollama/health');
                    const data = await response.json();
                    
                    const statusDiv = document.getElementById('ollama-status');
                    if (data.ollama_running) {
                        statusDiv.innerHTML = `
                            <h3>✅ Ollama פועל</h3>
                            <p>📊 מספר מודלים: ${data.models_count}</p>
                            <p>🤖 מודלים מומלצים: ${data.default_models.join(', ')}</p>
                        `;
                        statusDiv.style.background = '#e8f5e9';
                    } else {
                        statusDiv.innerHTML = `
                            <h3>❌ Ollama לא פועל</h3>
                            <p>אנא ודא ש-Ollama רץ על localhost:11434</p>
                            <p>הפעל עם: <code>ollama serve</code></p>
                        `;
                        statusDiv.style.background = '#ffebee';
                    }
                }

                async function listModels() {
                    const response = await fetch('/api/ollama/models');
                    const data = await response.json();
                    
                    const modelsDiv = document.getElementById('models-list');
                    if (data.success) {
                        let html = '<h3>📚 מודלים זמינים:</h3><ul>';
                        data.models.forEach(model => {
                            const sizeGB = (model.size / 1024 / 1024 / 1024).toFixed(1);
                            html += `<li><strong>${model.name}</strong> - ${sizeGB} GB</li>`;
                        });
                        html += '</ul>';
                        modelsDiv.innerHTML = html;
                    } else {
                        modelsDiv.innerHTML = `<p class="error">❌ שגיאה: ${data.error}</p>`;
                    }
                }

                async function generateText() {
                    const model = document.getElementById('model').value;
                    const prompt = document.getElementById('prompt').value;
                    const taskType = document.getElementById('task-type').value;
                    const language = document.getElementById('language').value;
                    
                    const responseDiv = document.getElementById('generation-response');
                    responseDiv.innerHTML = '<p>⚡ מעבד...</p>';
                    
                    const requestBody = {
                        model: model,
                        prompt: prompt,
                        task_type: taskType,
                        language: taskType === 'code' ? language : null
                    };
                    
                    try {
                        const response = await fetch('/api/ollama/generate', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(requestBody)
                        });
                        
                        const data = await response.json();
                        
                        if (data.success) {
                            responseDiv.innerHTML = `
                                <h3>✅ תשובה:</h3>
                                <p><strong>מודל:</strong> ${data.model}</p>
                                <p><strong>זמן עיבוד:</strong> ${data.processing_time_ms} ms</p>
                                <div style="background: white; padding: 15px; border-radius: 5px; margin-top: 10px;">
                                    ${data.response.replace(/\n/g, '<br>')}
                                </div>
                            `;
                            responseDiv.className = 'response';
                        } else {
                            responseDiv.innerHTML = `<p class="error">❌ שגיאה: ${data.error}</p>`;
                            responseDiv.className = 'response error';
                        }
                    } catch (error) {
                        responseDiv.innerHTML = `<p class="error">❌ שגיאת רשת: ${error}</p>`;
                        responseDiv.className = 'response error';
                    }
                }

                // Initialize on page load
                document.addEventListener('DOMContentLoaded', () => {
                    checkHealth();
                    listModels();
                });
            </script>
        </head>
        <body>
            <div class="header">
                <h1>🤖 CalcLaw - Ollama Integration</h1>
                <p>שילוב מודלי AI מקומיים עם CalcLaw</p>
            </div>
            
            <div id="ollama-status" class="status">
                <p>בודק סטטוס...</p>
            </div>
            
            <div class="feature">
                <h3>🤖 יצירת טקסט עם AI מקומי</h3>
                <div class="form-group">
                    <label for="model">מודל:</label>
                    <select id="model">
                        <option value="phi3:mini">Phi3 Mini (2.2GB) - קל ומהיר</option>
                        <option value="gemma2:9b">Gemma2:9b (5.4GB) - חזק</option>
                        <option value="phi3:3.8b">Phi3:3.8b (2.2GB) - כללי</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="task-type">סוג משימה:</label>
                    <select id="task-type" onchange="toggleLanguageField()">
                        <option value="general">כללי</option>
                        <option value="hebrew">עברית</option>
                        <option value="code">קוד</option>
                    </select>
                </div>
                
                <div class="form-group" id="language-field" style="display: none;">
                    <label for="language">שפת תכנות:</label>
                    <input type="text" id="language" value="Python" placeholder="Python, JavaScript, Rust, etc.">
                </div>
                
                <div class="form-group">
                    <label for="prompt">פקודה:</label>
                    <textarea id="prompt" rows="4" placeholder="מה תרצה ליצור?"></textarea>
                </div>
                
                <button onclick="generateText()">🚀 צור טקסט</button>
            </div>
            
            <div id="generation-response" class="response">
                <!-- Response will appear here -->
            </div>
            
            <div id="models-list" class="feature">
                <h3>📚 טוען רשימת מודלים...</h3>
            </div>
            
            <div class="api-section">
                <h3>🔗 API Endpoints</h3>
                <div class="hebrew">
                    <p><strong>GET</strong> <span class="endpoint">/api/ollama/health</span> - בדיקת סטטוס Ollama</p>
                    <p><strong>GET</strong> <span class="endpoint">/api/ollama/models</span> - רשימת מודלים זמינים</p>
                    <p><strong>POST</strong> <span class="endpoint">/api/ollama/generate</span> - יצירת טקסט עם מודל</p>
                </div>
            </div>
            
            <script>
                function toggleLanguageField() {
                    const taskType = document.getElementById('task-type').value;
                    const languageField = document.getElementById('language-field');
                    languageField.style.display = taskType === 'code' ? 'block' : 'none';
                }
            </script>
        </body>
        </html>
    "#)
}

// Create router for Ollama endpoints
pub fn create_ollama_router(state: Arc<OllamaAppState>) -> Router {
    Router::new()
        .route("/api/ollama/health", get(ollama_health))
        .route("/api/ollama/models", get(list_models))
        .route("/api/ollama/generate", post(generate_text))
        .route("/ollama", get(ollama_dashboard))
        .with_state(state)
}