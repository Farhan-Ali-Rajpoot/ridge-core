use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Default)]
pub struct EdgeConfig {
    pub enabled: bool,
    pub platform: String,
    pub kv_bindings: HashMap<String,String>,
    pub geo_enabled: bool,
    pub extensions: HashMap<String, Value>
}