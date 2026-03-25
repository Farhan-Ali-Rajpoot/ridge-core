use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;

use crate::core::OmnyxCore;


#[async_trait]
pub trait RealtimeAdapter: Send + Sync + 'static {
    async fn subcribe(&self, core: &OmnyxCore, topic: &str) -> Result<Arc<dyn RealtimeSubcription>, String>;
    fn extension(&self, key: &str) -> Option<Value>;
}

#[async_trait]
pub trait RealtimeSubcription: Send + Sync + 'static {
    async fn send(&self, data: Value);
}