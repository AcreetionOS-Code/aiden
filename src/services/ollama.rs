use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct OllamaService {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatResponse {
    message: OllamaMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct EmbedResponse {
    embedding: Vec<f32>,
}

impl OllamaService {
    pub fn new(host: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap();
        
        Self {
            client,
            base_url: format!("http://{}", host),
        }
    }
    
    pub async fn chat(&self, model: &str, messages: &[OllamaMessage]) -> Result<String> {
        let response = self.client
            .post(format!("{}/api/chat", self.base_url))
            .json(&serde_json::json!({
                "model": model,
                "messages": messages,
                "stream": false
            }))
            .send()
            .await?;
        
        let response: ChatResponse = response.json().await?;
        Ok(response.message.content)
    }
    
    pub async fn embeddings(&self, model: &str, text: &str) -> Result<Vec<f32>> {
        let response = self.client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&serde_json::json!({
                "model": model,
                "prompt": text
            }))
            .send()
            .await?;
        
        let response: EmbedResponse = response.json().await?;
        Ok(response.embedding)
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        match self.client
            .get(format!("{}/", self.base_url))
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
