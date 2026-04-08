// Text-to-Speech module for CalcLaw
// Supports multiple TTS providers with Hebrew support

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

// TTS Provider types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TtsProvider {
    ElevenLabs,
    GoogleCloud,
    Azure,
    OpenAITts,
    Local, // For local TTS engines like Piper, Coqui
}

// Voice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub provider: TtsProvider,
    pub voice_id: String,
    pub language: String,
    pub speed: f32, // 0.5 to 2.0
    pub pitch: f32, // -20 to 20 for some providers
    pub volume: f32, // 0.0 to 1.0
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            provider: TtsProvider::GoogleCloud,
            voice_id: "he-IL-Standard-A".to_string(), // Hebrew female voice
            language: "he-IL".to_string(),
            speed: 1.0,
            pitch: 0.0,
            volume: 1.0,
        }
    }
}

// TTS Request
#[derive(Debug, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice_config: Option<VoiceConfig>,
    pub output_format: String, // mp3, wav, ogg
    pub save_to_file: bool,
    pub filename: Option<String>,
}

// TTS Response
#[derive(Debug, Serialize, Deserialize)]
pub struct TtsResponse {
    pub success: bool,
    pub audio_data: Option<Vec<u8>>,
    pub file_path: Option<String>,
    pub duration_ms: Option<u32>,
    pub provider: String,
    pub error: Option<String>,
}

// TTS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub default_provider: TtsProvider,
    pub api_keys: TtsApiKeys,
    pub cache_dir: PathBuf,
    pub max_audio_size_mb: u32,
    pub default_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsApiKeys {
    pub elevenlabs: Option<String>,
    pub google_cloud: Option<String>,
    pub azure: Option<String>,
    pub openai: Option<String>,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            default_provider: TtsProvider::GoogleCloud,
            api_keys: TtsApiKeys {
                elevenlabs: None,
                google_cloud: None,
                azure: None,
                openai: None,
            },
            cache_dir: PathBuf::from("./tts_cache"),
            max_audio_size_mb: 10,
            default_language: "he-IL".to_string(),
        }
    }
}

// Available voices by provider and language
pub const AVAILABLE_VOICES: &[(&str, &str, &str, &str)] = &[
    // Google Cloud TTS Hebrew voices
    ("GoogleCloud", "he-IL-Standard-A", "he-IL", "Hebrew Female"),
    ("GoogleCloud", "he-IL-Standard-B", "he-IL", "Hebrew Male"),
    ("GoogleCloud", "he-IL-Standard-C", "he-IL", "Hebrew Female 2"),
    ("GoogleCloud", "he-IL-Standard-D", "he-IL", "Hebrew Male 2"),
    
    // Google Cloud English voices
    ("GoogleCloud", "en-US-Standard-A", "en-US", "US English Female"),
    ("GoogleCloud", "en-US-Standard-B", "en-US", "US English Male"),
    ("GoogleCloud", "en-US-Standard-C", "en-US", "US English Female 2"),
    ("GoogleCloud", "en-US-Standard-D", "en-US", "US English Male 2"),
    
    // ElevenLabs voices (example IDs)
    ("ElevenLabs", "21m00Tcm4TlvDq8ikWAM", "en-US", "Rachel"),
    ("ElevenLabs", "AZnzlk1XvdvUeBnXmlld", "en-US", "Domi"),
    ("ElevenLabs", "EXAVITQu4vr4xnSDxMaL", "en-US", "Bella"),
    
    // Azure TTS voices
    ("Azure", "he-IL-Hila", "he-IL", "Hebrew Female - Hila"),
    ("Azure", "he-IL-Avri", "he-IL", "Hebrew Male - Avri"),
    ("Azure", "en-US-JennyNeural", "en-US", "US English Female - Jenny"),
    ("Azure", "en-US-GuyNeural", "en-US", "US English Male - Guy"),
    
    // OpenAI TTS voices
    ("OpenAITts", "alloy", "en", "Alloy"),
    ("OpenAITts", "echo", "en", "Echo"),
    ("OpenAITts", "fable", "en", "Fable"),
    ("OpenAITts", "onyx", "en", "Onyx"),
    ("OpenAITts", "nova", "en", "Nova"),
    ("OpenAITts", "shimmer", "en", "Shimmer"),
];

// Main TTS Client
pub struct TtsClient {
    config: Arc<RwLock<TtsConfig>>,
    client: Client,
}

impl TtsClient {
    pub fn new(config: TtsConfig) -> Result<Self> {
        // Create cache directory if it doesn't exist
        if !config.cache_dir.exists() {
            std::fs::create_dir_all(&config.cache_dir)
                .map_err(|e| anyhow!("Failed to create TTS cache directory: {}", e))?;
        }

        let client = Client::builder()
            .user_agent("CalcLaw-TTS/0.1.0")
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            client,
        })
    }

    pub async fn update_config(&self, new_config: TtsConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    pub async fn get_config(&self) -> TtsConfig {
        let config = self.config.read().await;
        config.clone()
    }

    // Generate speech from text
    pub async fn generate_speech(&self, request: TtsRequest) -> Result<TtsResponse> {
        let config = self.config.read().await;
        let voice_config = request.voice_config.unwrap_or_else(|| {
            VoiceConfig {
                provider: config.default_provider.clone(),
                language: config.default_language.clone(),
                ..VoiceConfig::default()
            }
        });

        info!("Generating TTS for {} chars with {:?}", request.text.len(), voice_config.provider);

        match voice_config.provider {
            TtsProvider::GoogleCloud => self.google_tts(&request, &voice_config, &config).await,
            TtsProvider::ElevenLabs => self.elevenlabs_tts(&request, &voice_config, &config).await,
            TtsProvider::Azure => self.azure_tts(&request, &voice_config, &config).await,
            TtsProvider::OpenAITts => self.openai_tts(&request, &voice_config, &config).await,
            TtsProvider::Local => self.local_tts(&request, &voice_config).await,
        }
    }

    // Google Cloud TTS
    async fn google_tts(
        &self,
        request: &TtsRequest,
        voice_config: &VoiceConfig,
        config: &TtsConfig,
    ) -> Result<TtsResponse> {
        let api_key = config.api_keys.google_cloud.as_ref()
            .ok_or_else(|| anyhow!("Google Cloud TTS API key not configured"))?;

        let url = format!(
            "https://texttospeech.googleapis.com/v1/text:synthesize?key={}",
            api_key
        );

        let request_body = serde_json::json!({
            "input": {
                "text": request.text
            },
            "voice": {
                "languageCode": voice_config.language,
                "name": voice_config.voice_id,
                "ssmlGender": "FEMALE" // Will need to map from voice_id
            },
            "audioConfig": {
                "audioEncoding": "MP3",
                "speakingRate": voice_config.speed,
                "pitch": voice_config.pitch,
                "volumeGainDb": (voice_config.volume - 1.0) * 6.0, // Convert 0-1 to dB
            }
        });

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Google TTS API error: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Google TTS API error: {}", error_text));
        }

        let api_response: serde_json::Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse Google TTS response: {}", e))?;

        let audio_content = api_response["audioContent"]
            .as_str()
            .ok_or_else(|| anyhow!("No audio content in response"))?;

        let audio_data = base64::decode(audio_content)
            .map_err(|e| anyhow!("Failed to decode audio content: {}", e))?;

        // Check size limit
        if audio_data.len() > (config.max_audio_size_mb as usize) * 1024 * 1024 {
            return Err(anyhow!("Audio file too large ({} MB)", audio_data.len() / 1024 / 1024));
        }

        let file_path = if request.save_to_file {
            let filename = request.filename.clone().unwrap_or_else(|| {
                format!("tts_{}.mp3", chrono::Utc::now().timestamp())
            });
            let path = config.cache_dir.join(&filename);
            std::fs::write(&path, &audio_data)
                .map_err(|e| anyhow!("Failed to save audio file: {}", e))?;
            Some(path.to_string_lossy().to_string())
        } else {
            None
        };

        Ok(TtsResponse {
            success: true,
            audio_data: Some(audio_data),
            file_path,
            duration_ms: None, // Would need to calculate from audio
            provider: "GoogleCloud".to_string(),
            error: None,
        })
    }

    // ElevenLabs TTS
    async fn elevenlabs_tts(
        &self,
        request: &TtsRequest,
        voice_config: &VoiceConfig,
        config: &TtsConfig,
    ) -> Result<TtsResponse> {
        let api_key = config.api_keys.elevenlabs.as_ref()
            .ok_or_else(|| anyhow!("ElevenLabs API key not configured"))?;

        let url = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{}",
            voice_config.voice_id
        );

        let request_body = serde_json::json!({
            "text": request.text,
            "model_id": "eleven_multilingual_v2", // Supports Hebrew
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75,
                "style": 0.0,
                "use_speaker_boost": true
            }
        });

        let response = self.client
            .post(&url)
            .header("xi-api-key", api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("ElevenLabs API error: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("ElevenLabs API error: {}", error_text));
        }

        let audio_data = response.bytes().await
            .map_err(|e| anyhow!("Failed to read audio data: {}", e))?
            .to_vec();

        // Similar file handling as Google TTS
        let file_path = self.handle_audio_file(request, &audio_data, config).await?;

        Ok(TtsResponse {
            success: true,
            audio_data: Some(audio_data),
            file_path,
            duration_ms: None,
            provider: "ElevenLabs".to_string(),
            error: None,
        })
    }

    // Azure TTS
    async fn azure_tts(
        &self,
        request: &TtsRequest,
        voice_config: &VoiceConfig,
        config: &TtsConfig,
    ) -> Result<TtsResponse> {
        let api_key = config.api_keys.azure.as_ref()
            .ok_or_else(|| anyhow!("Azure TTS API key not configured"))?;

        let region = "eastus"; // Would need to be configurable
        let url = format!(
            "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
            region
        );

        // SSML for better control
        let ssml = format!(
            r#"<speak version="1.0" xml:lang="{}">
                <voice name="{}">
                    <prosody rate="{}" pitch="{}Hz" volume="{}">
                        {}
                    </prosody>
                </voice>
            </speak>"#,
            voice_config.language,
            voice_config.voice_id,
            voice_config.speed,
            (voice_config.pitch + 100.0).max(50.0).min(200.0),
            voice_config.volume,
            request.text
        );

        let response = self.client
            .post(&url)
            .header("Ocp-Apim-Subscription-Key", api_key)
            .header("Content-Type", "application/ssml+xml")
            .header("X-Microsoft-OutputFormat", "audio-24khz-96kbitrate-mono-mp3")
            .body(ssml)
            .send()
            .await
            .map_err(|e| anyhow!("Azure TTS API error: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Azure TTS API error: {}", error_text));
        }

        let audio_data = response.bytes().await
            .map_err(|e| anyhow!("Failed to read audio data: {}", e))?
            .to_vec();

        let file_path = self.handle_audio_file(request, &audio_data, config).await?;

        Ok(TtsResponse {
            success: true,
            audio_data: Some(audio_data),
            file_path,
            duration_ms: None,
            provider: "Azure".to_string(),
            error: None,
        })
    }

    // OpenAI TTS
    async fn openai_tts(
        &self,
        request: &TtsRequest,
        voice_config: &VoiceConfig,
        config: &TtsConfig,
    ) -> Result<TtsResponse> {
        let api_key = config.api_keys.openai.as_ref()
            .ok_or_else(|| anyhow!("OpenAI TTS API key not configured"))?;

        let url = "https://api.openai.com/v1/audio/speech";

        let request_body = serde_json::json!({
            "model": "tts-1",
            "input": request.text,
            "voice": voice_config.voice_id,
            "speed": voice_config.speed,
            "response_format": "mp3"
        });

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("OpenAI TTS API error: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenAI TTS API error: {}", error_text));
        }

        let audio_data = response.bytes().await
            .map_err(|e| anyhow!("Failed to read audio data: {}", e))?
            .to_vec();

        let file_path = self.handle_audio_file(request, &audio_data, config).await?;

        Ok(TtsResponse {
            success: true,
            audio_data: Some(audio_data),
            file_path,
            duration_ms: None,
            provider: "OpenAI".to_string(),
            error: None,
        })
    }

    // Local TTS (e.g., Piper, Coqui)
    async fn local_tts(
        &self,
        request: &TtsRequest,
        voice_config: &VoiceConfig,
    ) -> Result<TtsResponse> {
        // This would integrate with local TTS engines
        // For now, return a placeholder
        Err(anyhow!("Local TTS not yet implemented. Use Google Cloud, ElevenLabs, Azure, or OpenAI."))
    }

    // Helper to handle audio file saving
    async fn handle_audio_file(
        &self,
        request: &TtsRequest,
        audio_data: &[u8],
        config: &TtsConfig,
    ) -> Result<Option<String>> {
        if request.save_to_file {
            let filename = request.filename.clone().unwrap_or_else(|| {
                format!("tts_{}.mp3", chrono::Utc::now().timestamp())
            });
            let path = config.cache_dir.join