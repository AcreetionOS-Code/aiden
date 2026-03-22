use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;
use uuid::Uuid;
use qdrant_client::qdrant::PointStruct;
use crate::services::OllamaService;
use crate::services::QdrantService;

pub struct IndexerService {
    ollama: OllamaService,
    qdrant: QdrantService,
    docs_path: String,
    chunk_size: usize,
    embed_model: String,
}

impl IndexerService {
    pub fn new(
        ollama: OllamaService,
        qdrant: QdrantService,
        docs_path: String,
        chunk_size: usize,
        embed_model: String,
    ) -> Self {
        Self {
            ollama,
            qdrant,
            docs_path,
            chunk_size,
            embed_model,
        }
    }
    
    pub async fn index_all(&self) -> Result<IndexStats> {
        let mut stats = IndexStats {
            files_processed: 0,
            chunks_created: 0,
            errors: Vec::new(),
        };
        
        self.qdrant.init_collection().await?;
        
        let docs_path = Path::new(&self.docs_path);
        
        if !docs_path.exists() {
            return Err(anyhow::anyhow!("Docs path does not exist: {:?}", docs_path));
        }
        
        for entry in WalkDir::new(docs_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
                match self.process_file(path).await {
                    Ok(chunks) => {
                        stats.files_processed += 1;
                        stats.chunks_created += chunks.len();
                        self.upsert_chunks(chunks).await?;
                    }
                    Err(e) => {
                        stats.errors.push(format!("{}: {}", path.display(), e));
                    }
                }
            }
        }
        
        Ok(stats)
    }
    
    async fn process_file(&self, path: &Path) -> Result<Vec<Chunk>> {
        let content = std::fs::read_to_string(path)?;
        let relative_path = path.strip_prefix(&self.docs_path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        let chunks = self.chunk_text(&content);
        
        let chunks: Vec<Chunk> = chunks
            .into_iter()
            .enumerate()
            .map(|(i, text)| Chunk {
                id: Uuid::new_v4().to_string(),
                text,
                source: relative_path.clone(),
                chunk_index: i,
            })
            .collect();
        
        Ok(chunks)
    }
    
    fn chunk_text(&self, text: &str) -> Vec<String> {
        let sentences: Vec<&str> = text
            .split(&['.', '!', '?', '\n'][..])
            .collect();
        
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_size = 0;
        
        for sentence in sentences {
            let sentence = sentence.trim();
            if sentence.is_empty() {
                continue;
            }
            
            let sentence_size = sentence.split_whitespace().count();
            
            if current_size + sentence_size > self.chunk_size && current_size > 0 {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
                current_size = 0;
            }
            
            if !current_chunk.is_empty() {
                current_chunk.push_str(". ");
            }
            current_chunk.push_str(sentence);
            current_size += sentence_size;
        }
        
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }
        
        chunks
    }
    
    async fn upsert_chunks(&self, chunks: Vec<Chunk>) -> Result<()> {
        let mut points = Vec::new();
        
        for chunk in chunks {
            let embedding = self.ollama.embeddings(&self.embed_model, &chunk.text).await?;
            
            let point = PointStruct::new(
                chunk.id.clone(),
                embedding,
                [
                    ("text", chunk.text.clone().into()),
                    ("source", chunk.source.clone().into()),
                    ("chunk_index", (chunk.chunk_index as i64).into()),
                ],
            );
            
            points.push(point);
        }
        
        self.qdrant.upsert(points).await?;
        
        Ok(())
    }
}

pub struct Chunk {
    pub id: String,
    pub text: String,
    pub source: String,
    pub chunk_index: usize,
}

pub struct IndexStats {
    pub files_processed: usize,
    pub chunks_created: usize,
    pub errors: Vec<String>,
}
