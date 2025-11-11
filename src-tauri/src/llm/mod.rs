use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config::LLMConfig;
use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// Trait for LLM backends
#[async_trait]
pub trait LLMBackend: Send + Sync {
    async fn rewrite(&self, text: &str, prompt: &str) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}

/// Unified OpenAI-compatible client (works with OpenAI and Ollama)
pub struct UnifiedLLMClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

impl UnifiedLLMClient {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
            model,
            temperature,
            max_tokens,
        }
    }
}

#[async_trait]
impl LLMBackend for UnifiedLLMClient {
    async fn rewrite(&self, text: &str, prompt: &str) -> Result<String> {
        log::info!("Rewriting text with {} ({})", self.name(), self.model);

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let url = format!("{}/chat/completions", self.base_url);
        let mut req = self.client.post(&url).json(&request);

        // Only add auth header if API key is not "ollama" (Ollama doesn't need it)
        if self.api_key != "ollama" && !self.api_key.is_empty() {
            req = req.bearer_auth(&self.api_key);
        }

        let response = req
            .send()
            .await
            .map_err(|e| AppError::LLMProcessing(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::LLMProcessing(format!(
                "API error ({}): {}",
                status, error_text
            )));
        }

        let result: ChatCompletionResponse = response.json().await?;
        let rewritten_text = result
            .choices
            .first()
            .ok_or_else(|| AppError::LLMProcessing("No response from LLM".to_string()))?
            .message
            .content
            .clone();

        log::info!("Rewriting complete: {} characters", rewritten_text.len());
        Ok(rewritten_text)
    }

    async fn is_available(&self) -> bool {
        // For Ollama (local), check if server is running
        if self.base_url.contains("localhost") || self.base_url.contains("127.0.0.1") {
            self.client
                .get(format!("{}/api/tags", self.base_url.replace("/v1", "")))
                .send()
                .await
                .is_ok()
        } else {
            // For cloud APIs, assume available if API key is set
            !self.api_key.is_empty() && self.api_key != "ollama"
        }
    }

    fn name(&self) -> &str {
        if self.base_url.contains("localhost") || self.base_url.contains("127.0.0.1") {
            "Ollama"
        } else {
            "OpenAI"
        }
    }
}

/// LLM service with prompt templates
pub struct LLMService {
    backend: Option<Box<dyn LLMBackend>>,
    default_prompt: String,
}

impl LLMService {
    pub fn new(config: &LLMConfig) -> Result<Self> {
        let backend: Option<Box<dyn LLMBackend>> = match config.backend {
            crate::config::LLMBackend::None => None,
            _ => {
                let api_key = config
                    .api_key
                    .clone()
                    .unwrap_or_else(|| "ollama".to_string());
                Some(Box::new(UnifiedLLMClient::new(
                    config.base_url.clone(),
                    api_key,
                    config.model.clone(),
                    config.temperature,
                    config.max_tokens,
                )))
            }
        };

        let default_prompt = Self::get_default_prompt(&config.default_template);

        Ok(Self {
            backend,
            default_prompt,
        })
    }

    pub async fn rewrite_text(&self, text: &str) -> Result<String> {
        match &self.backend {
            Some(backend) => {
                if !backend.is_available().await {
                    log::warn!("LLM backend not available, returning original text");
                    return Ok(text.to_string());
                }
                backend.rewrite(text, &self.default_prompt).await
            }
            None => {
                log::info!("No LLM backend configured, returning original text");
                Ok(text.to_string())
            }
        }
    }

    fn get_default_prompt(template_name: &str) -> String {
        match template_name {
            "minimal" => {
                "Clean up this voice transcription by:\n\
                1. Removing filler words (um, uh, like, you know)\n\
                2. Fixing obvious typos\n\
                3. Adding basic punctuation\n\
                4. DO NOT change the tone or rephrase sentences\n\n\
                Transcription: ".to_string()
            }
            "professional" => {
                "You are a professional writing assistant. Transform this voice transcription into polished, professional business communication.\n\n\
                Guidelines:\n\
                - Remove all filler words and casual language\n\
                - Use formal, professional tone\n\
                - Improve sentence structure and flow\n\
                - Add appropriate business language\n\
                - Maintain clarity and conciseness\n\
                - Preserve factual content exactly\n\n\
                Transcription: ".to_string()
            }
            _ => {
                // Default: balanced
                "You are a text refinement assistant. Your task is to clean up voice transcriptions while preserving the original meaning and intent.\n\n\
                Instructions:\n\
                - Remove filler words (um, uh, like, you know, so, kind of)\n\
                - Fix grammar and punctuation\n\
                - Improve sentence structure slightly\n\
                - Keep the same level of formality\n\
                - Preserve technical terms and proper nouns exactly\n\
                - DO NOT summarize or significantly rewrite\n\n\
                Transcription: ".to_string()
            }
        }
    }
}
