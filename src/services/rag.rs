use anyhow::Result;
use crate::state::{Source, SourceType, ChatMessage};
use crate::services::{OllamaService, QdrantService, OllamaMessage};
use std::fmt;

#[derive(Clone)]
pub struct RAGService {
    ollama: OllamaService,
    qdrant: QdrantService,
    chat_model: String,
    embed_model: String,
    threshold: f32,
    max_results: usize,
}

impl fmt::Debug for RAGService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RAGService")
            .field("chat_model", &self.chat_model)
            .field("embed_model", &self.embed_model)
            .field("threshold", &self.threshold)
            .field("max_results", &self.max_results)
            .finish()
    }
}

impl RAGService {
    pub fn new(
        ollama: OllamaService,
        qdrant: QdrantService,
        chat_model: String,
        embed_model: String,
        threshold: f32,
        max_results: usize,
    ) -> Self {
        Self {
            ollama,
            qdrant,
            chat_model,
            embed_model,
            threshold,
            max_results,
        }
    }
    
    pub async fn query(&self, question: &str) -> Result<ChatMessage> {
        let query_embedding = self.ollama
            .embeddings(&self.embed_model, question)
            .await?;
        
        let local_hits = self.qdrant
            .search(&query_embedding, self.max_results)
            .await?;
        
        let top_score = local_hits.first().map(|h| h.score).unwrap_or(0.0);
        let needs_web = top_score < self.threshold;
        
        let mut all_sources = Vec::new();
        let mut context_parts = Vec::new();
        
        for (i, hit) in local_hits.iter().enumerate() {
            if hit.score >= 0.3 {
                context_parts.push(format!("[{}] {}", i + 1, hit.text));
                all_sources.push(Source {
                    source_type: SourceType::Local,
                    title: hit.source.clone(),
                    url: format!("file://{}", hit.source),
                    content: hit.text.clone(),
                    score: hit.score,
                });
            }
        }
        
        let context = context_parts.join("\n\n");
        
        let system_prompt = format!(
            "You are AIDEN, a helpful assistant for AcreetionOS, a user-friendly Arch-based Linux distribution. Use the following context to answer the user's question. If the context doesn't contain the answer, say so based on your knowledge of AcreetionOS. Be helpful, concise, and friendly.\n\nContext:\n{}",
            context
        );
        
        let messages = vec![
            OllamaMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            OllamaMessage {
                role: "user".to_string(),
                content: question.to_string(),
            },
        ];
        
        let response = self.ollama.chat(&self.chat_model, &messages).await?;
        
        Ok(ChatMessage::assistant(
            response,
            all_sources,
            needs_web && top_score < 0.5,
        ))
    }
}
