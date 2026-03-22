use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: Role,
    pub content: String,
    pub sources: Vec<Source>,
    pub used_web_fallback: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub source_type: SourceType,
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    Local,
    Web,
}

impl ChatMessage {
    #[allow(dead_code)]
    pub fn user(content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: Role::User,
            content,
            sources: Vec::new(),
            used_web_fallback: false,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn assistant(content: String, sources: Vec<Source>, used_web_fallback: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: Role::Assistant,
            content,
            sources,
            used_web_fallback,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
