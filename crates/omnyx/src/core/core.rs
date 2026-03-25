use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

use crate::render::ResponsePipeline;
use crate::core::router::tree::RouteTree;
use crate::plugin::OmnyxPlugin;
use crate::realtime::RealtimeAdapter;
use crate::edge::EdgeConfig;
use crate::client::ClientMirrorConfig;
use crate::diagnostics::DiagnosticsConfig;


#[derive(Clone)]
pub struct OmnyxCore {
    pub routes: Arc<RouteTree>,
    pub renderer: Arc<ResponsePipeline>,
    pub plugins: Vec<Arc<dyn OmnyxPlugin>>,
    pub realtime: Vec<Arc<dyn RealtimeAdapter>>,
    pub edge_config: EdgeConfig,
    pub client_mirror: ClientMirrorConfig,
    pub diagnostics: DiagnosticsConfig,
}