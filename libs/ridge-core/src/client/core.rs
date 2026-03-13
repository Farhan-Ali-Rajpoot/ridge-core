use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Default)]
pub struct ClientMirrorConfig {
    pub enabled: bool,
    pub wasm_bundle_path: String,
    pub include_metadata: bool,
    pub include_tree: bool,
    pub minify: bool,
    pub extensions: HashMap<String, Value>,
}