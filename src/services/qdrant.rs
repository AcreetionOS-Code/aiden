use anyhow::Result;
use qdrant_client::Qdrant;
use std::sync::Arc;

#[derive(Clone)]
pub struct QdrantService {
    client: Arc<Qdrant>,
    collection_name: String,
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    #[allow(dead_code)]
    pub id: String,
    pub score: f32,
    pub text: String,
    pub source: String,
    #[allow(dead_code)]
    pub chunk_index: usize,
}

impl QdrantService {
    pub async fn new(collection_name: &str) -> Result<Self> {
        let client = Qdrant::from_url("http://localhost:6334").build()?;

        Ok(Self {
            client: Arc::new(client),
            collection_name: collection_name.to_string(),
        })
    }

    pub async fn init_collection(&self) -> Result<()> {
        use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};

        let collection_exists = self.client.collection_exists(&self.collection_name).await?;

        if !collection_exists {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(&self.collection_name)
                        .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine)),
                )
                .await?;

            tracing::info!("Created Qdrant collection: {}", self.collection_name);
        }

        Ok(())
    }

    pub async fn search(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchHit>> {
        use qdrant_client::qdrant::QueryPointsBuilder;

        let response = self
            .client
            .query(
                QueryPointsBuilder::new(&self.collection_name)
                    .query(embedding.to_vec())
                    .limit(limit as u64)
                    .with_payload(true),
            )
            .await?;

        let hits: Vec<SearchHit> = response
            .result
            .into_iter()
            .map(|point| {
                let payload = point.payload;
                let text = payload
                    .get("text")
                    .and_then(|p| p.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                let source = payload
                    .get("source")
                    .and_then(|p| p.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                let chunk_index = payload
                    .get("chunk_index")
                    .and_then(|p| p.as_integer())
                    .unwrap_or(0) as usize;

                let id = point
                    .id
                    .and_then(|id| {
                        id.point_id_options.map(|opt| match opt {
                            qdrant_client::qdrant::point_id::PointIdOptions::Num(n) => {
                                n.to_string()
                            }
                            qdrant_client::qdrant::point_id::PointIdOptions::Uuid(s) => s,
                        })
                    })
                    .unwrap_or_default();

                SearchHit {
                    id,
                    score: point.score,
                    text,
                    source,
                    chunk_index,
                }
            })
            .collect();

        Ok(hits)
    }

    pub async fn upsert(&self, points: Vec<qdrant_client::qdrant::PointStruct>) -> Result<()> {
        use qdrant_client::qdrant::UpsertPointsBuilder;

        self.client
            .upsert_points(UpsertPointsBuilder::new(&self.collection_name, points))
            .await?;

        Ok(())
    }
}
