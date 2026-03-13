use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::{http::HeaderMap, response::Response};
use serde_json::Value;

use crate::core::RidgeCore;
use crate::core::router::metadata::RouteMetadata;


#[async_trait]
pub trait ResponseStrategy: Send + Sync + 'static  {

    async fn applies(&self, headers: &HeaderMap, core: &RidgeCore) -> bool; 

    async fn respond(
        &self,
        inner_html: String,
        metadata: Arc<RouteMetadata>,
        core: &RidgeCore,
        original_response: Response,
    ) -> Response;
}

#[derive(Clone)]
pub struct ResponsePipeline {
    pub strategies: Vec<Arc<dyn ResponseStrategy>>,
    pub fallback: Option<Arc<dyn ResponseStrategy>>,

    pub mode_configs: HashMap<String, ModeConfig>,
    
    pub extensions: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct ModeConfig {
    pub flags: HashMap<String, bool>,
    pub numerics: HashMap<String, u64>,
    pub strings: HashMap<String, String>,
    pub extensions: HashMap<String, Value>
}

