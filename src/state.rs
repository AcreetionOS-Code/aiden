pub mod message;

pub use message::*;

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ollama: OllamaConfig,
    pub search: SearchConfig,
    pub indexing: IndexingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub host: String,
    pub chat_model: String,
    pub embed_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub threshold: f32,
    pub max_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    pub docs_path: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub collection_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama: OllamaConfig {
                host: "localhost:11434".to_string(),
                chat_model: "llama3.1:8b".to_string(),
                embed_model: "nomic-embed-text".to_string(),
            },
            search: SearchConfig {
                enabled: true,
                api_key: None,
                threshold: 0.7,
                max_results: 5,
            },
            indexing: IndexingConfig {
                docs_path: "./docs".to_string(),
                chunk_size: 512,
                chunk_overlap: 50,
                collection_name: "aiden".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    #[allow(dead_code)]
    pub messages: VecDeque<ChatMessage>,
    pub indexing_status: Option<IndexingStatus>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            messages: VecDeque::new(),
            indexing_status: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingStatus {
    pub is_indexing: bool,
    pub progress: f32,
    pub files_processed: usize,
    pub chunks_created: usize,
    pub current_file: String,
    pub errors: Vec<String>,
}

impl Default for IndexingStatus {
    fn default() -> Self {
        Self {
            is_indexing: false,
            progress: 0.0,
            files_processed: 0,
            chunks_created: 0,
            current_file: String::new(),
            errors: Vec::new(),
        }
    }
}
