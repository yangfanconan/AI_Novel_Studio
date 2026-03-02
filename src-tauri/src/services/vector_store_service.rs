use serde::{Deserialize, Serialize};
use chrono::Utc;
use rusqlite::{Connection, params};

use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorChunk {
    pub id: String,
    pub chapter_id: String,
    pub chunk_index: i32,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub chunk_id: String,
    pub chapter_id: String,
    pub content: String,
    pub score: f32,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub embedding_dimension: usize,
}

impl Default for VectorStoreConfig {
    fn default() -> Self {
        Self {
            chunk_size: 500,
            chunk_overlap: 50,
            embedding_dimension: 1536,
        }
    }
}

pub struct VectorStoreService {
    config: VectorStoreConfig,
    logger: Logger,
}

impl VectorStoreService {
    pub fn new() -> Self {
        Self {
            config: VectorStoreConfig::default(),
            logger: Logger::new().with_feature("vector_store_service"),
        }
    }

    pub fn with_config(config: VectorStoreConfig) -> Self {
        Self {
            config,
            logger: Logger::new().with_feature("vector_store_service"),
        }
    }

    fn chunk_text(&self, text: &str) -> Vec<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < chars.len() {
            let end = (start + self.config.chunk_size).min(chars.len());
            let chunk: String = chars[start..end].iter().collect();
            
            if !chunk.trim().is_empty() {
                chunks.push(chunk.trim().to_string());
            }
            
            start = if end < chars.len() {
                end - self.config.chunk_overlap
            } else {
                break;
            };
        }

        chunks
    }

    pub async fn vectorize_chapter(&self, chapter_id: &str) -> Result<Vec<VectorChunk>, String> {
        self.logger.info(&format!("Vectorizing chapter: {}", chapter_id));

        Ok(Vec::new())
    }

    pub async fn vectorize_chapter_with_content(
        &self,
        conn: &Connection,
        chapter_id: &str,
        content: &str,
    ) -> Result<Vec<VectorChunk>, String> {
        self.logger.info(&format!("Vectorizing chapter with content: {}", chapter_id));

        let chunks = self.chunk_text(content);
        let mut vector_chunks = Vec::new();

        conn.execute(
            "DELETE FROM vector_chunks WHERE chapter_id = ?1",
            params![chapter_id],
        ).ok();

        for (i, chunk_content) in chunks.iter().enumerate() {
            let chunk_id = uuid::Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();

            conn.execute(
                "INSERT INTO vector_chunks (id, chapter_id, chunk_index, content, embedding, metadata, created_at) VALUES (?1, ?2, ?3, ?4, NULL, NULL, ?5)",
                params![&chunk_id, chapter_id, i as i32, chunk_content, &now],
            ).map_err(|e| format!("插入向量块失败: {}", e))?;

            vector_chunks.push(VectorChunk {
                id: chunk_id,
                chapter_id: chapter_id.to_string(),
                chunk_index: i as i32,
                content: chunk_content.clone(),
                embedding: None,
                metadata: None,
                created_at: now,
            });
        }

        self.logger.info(&format!("Created {} chunks for chapter: {}", vector_chunks.len(), chapter_id));
        Ok(vector_chunks)
    }

    pub async fn search_similar(
        &self,
        conn: &Connection,
        query: &str,
        project_id: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, String> {
        self.logger.info(&format!("Searching for: {}", query));

        let mut stmt = conn
            .prepare(
                "SELECT vc.id, vc.chapter_id, vc.content, vc.metadata 
                 FROM vector_chunks vc 
                 JOIN chapters c ON vc.chapter_id = c.id 
                 WHERE c.project_id = ?1 
                 ORDER BY vc.created_at DESC 
                 LIMIT ?2"
            )
            .map_err(|e| format!("准备查询失败: {}", e))?;

        let results = stmt
            .query_map(params![project_id, limit as i32], |row| {
                Ok(SearchResult {
                    chunk_id: row.get(0)?,
                    chapter_id: row.get(1)?,
                    content: row.get(2)?,
                    score: 0.5,
                    metadata: row.get(3)?,
                })
            })
            .map_err(|e| format!("搜索失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("解析结果失败: {}", e))?;

        let query_words: Vec<&str> = query.split_whitespace().collect();
        let mut scored_results: Vec<SearchResult> = results
            .into_iter()
            .map(|mut r| {
                let matches = query_words
                    .iter()
                    .filter(|word| r.content.to_lowercase().contains(&word.to_lowercase()))
                    .count();
                r.score = if query_words.is_empty() {
                    0.0
                } else {
                    matches as f32 / query_words.len() as f32
                };
                r
            })
            .collect();

        scored_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(scored_results)
    }

    pub async fn get_chapter_chunks(
        &self,
        conn: &Connection,
        chapter_id: &str,
    ) -> Result<Vec<VectorChunk>, String> {
        let mut stmt = conn
            .prepare(
                "SELECT id, chapter_id, chunk_index, content, embedding, metadata, created_at FROM vector_chunks WHERE chapter_id = ?1 ORDER BY chunk_index ASC"
            )
            .map_err(|e| format!("准备查询失败: {}", e))?;

        let chunks = stmt
            .query_map(params![chapter_id], |row| {
                Ok(VectorChunk {
                    id: row.get(0)?,
                    chapter_id: row.get(1)?,
                    chunk_index: row.get(2)?,
                    content: row.get(3)?,
                    embedding: None,
                    metadata: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| format!("获取向量块失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("解析向量块失败: {}", e))?;

        Ok(chunks)
    }

    pub async fn delete_chapter_chunks(
        &self,
        conn: &Connection,
        chapter_id: &str,
    ) -> Result<(), String> {
        conn.execute(
            "DELETE FROM vector_chunks WHERE chapter_id = ?1",
            params![chapter_id],
        ).map_err(|e| format!("删除向量块失败: {}", e))?;

        self.logger.info(&format!("Deleted chunks for chapter: {}", chapter_id));
        Ok(())
    }
}

impl Default for VectorStoreService {
    fn default() -> Self {
        Self::new()
    }
}
