use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Default)]
pub struct DiagnosticsConfig {
    pub tracing_enabled: bool,
    pub metrics_enabled: bool,
    pub log_level: String,           
    pub otel_endpoint: Option<String>,
    pub extensions: HashMap<String, Value>,
}