use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

use crate::core::RidgeCore;
use crate::core::router::RouteNode;


#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("fetched failed {0}")]
    Fetch(String),
    #[error("cache miss")]
    CacheMiss,
}

#[async_trait]
pub trait DataLoader: Send + Sync + 'static {

    async fn load(&self, core: &RidgeCore, params: &HashMap<String, String>) -> Result<LoaderOutput, LoaderError>;

    fn cache_config(&self) -> Option<CacheConfig>;
    fn extension(&self, key: &str) -> Option<Value>;
}

#[derive(Clone)]
pub struct LoaderOutput {
    pub data: Value,
    pub effects: HashMap<String, String>,
    pub revalidate: Vec<String>,
    pub extensions: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct CacheConfig {
    pub ttl: u64,    
    pub swr: u64,
    pub keys: Vec<String>,
    pub extensions: HashMap<String, Value>,
}