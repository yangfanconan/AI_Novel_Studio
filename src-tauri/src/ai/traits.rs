use crate::ai::models::{AIRequest, AIResponse, AIStreamChunk};
use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

pub struct ModelStream {
    inner: Box<dyn Stream<Item = Result<AIStreamChunk, String>> + Send + Unpin>,
}

impl ModelStream {
    pub fn new(stream: Box<dyn Stream<Item = Result<AIStreamChunk, String>> + Send + Unpin>) -> Self {
        ModelStream { inner: stream }
    }
}

impl Stream for ModelStream {
    type Item = Result<AIStreamChunk, String>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().inner).poll_next(cx)
    }
}

#[async_trait]
pub trait AIModel: Send + Sync {
    fn get_name(&self) -> String;
    fn get_provider(&self) -> String;
    
    async fn complete(&self, request: AIRequest) -> Result<AIResponse, String>;
    
    async fn complete_stream(&self, request: AIRequest) -> Result<ModelStream, String>;
}
