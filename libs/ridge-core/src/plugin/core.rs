use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;
use axum::response::Response;

use crate::core::RidgeCore;
use crate::core::router::RouteTree;
use crate::render::pipeline::ResponsePipeline;
use crate::core::router::metadata::RouteMetadata;


#[async_trait]
pub trait RidgePlugin: Send + Sync + 'static {

    async fn on_build(
        &self, 
        tree: &mut RouteTree,
        metadata: &mut RouteMetadata,
        pipeline: &mut ResponsePipeline,
    );

    async fn on_request(
        &self,
        core: &RidgeCore,
    ) -> Option<Response>;

    fn config(&self) -> Option<&PluginConfig>;
    fn extension(&self, key: &str) -> Option<Value>;
}

#[derive(Clone, Default)]
pub struct PluginConfig {
    pub flags: HashMap<String, bool>,
    pub numeric: HashMap<String, u64>,
    pub stings: HashMap<String, String>,
    pub nested: HashMap<String, String>,
    pub extensions: HashMap<String, Value>,
}