use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

use crate::core::RidgeCore;


#[derive(Clone)]
pub struct IslandConfig {
    pub id: String,
    pub bundle: Arc<String>,
    pub hydration: Option<HydrationStrategy>,
    pub props_source: PropsSource,
    pub deps: Vec<Arc<String>>,
    pub allowed_modes: Vec<String>,
    pub extensions: HashMap<String, Value>,
}

#[derive(Clone)]
pub enum HydrationStrategy {
    Eager,
    Lazy,
    Custom { script: String },
}

#[derive(Clone)]
pub enum PropsSource {
    Static(Value),
    FromLoader(String),
    Dynamic(Arc<dyn Fn(&RidgeCore) -> Value + Send + Sync>)
}