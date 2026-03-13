use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

use crate::core::router::metadata::RouteMetadata;
use crate::render::ResponsePipeline;
use crate::render::mode::RenderMode;
use crate::core::router::{RouteTree};
use crate::builder::route_tree_builder::RouteTreeBuilder;
use crate::plugin::RidgePlugin;
use crate::loader::DataLoader;
use crate::island::IslandConfig;
use crate::realtime::RealtimeAdapter;
use crate::edge::EdgeConfig;
use crate::client::ClientMirrorConfig;
use crate::diagnostics::DiagnosticsConfig;

#[derive(Clone)]
pub struct RidgeCore {
    pub axum_router: Arc<axum::Router>,
    pub route_tree: Arc<RouteTree>,

    pub builder: Arc<dyn RouteTreeBuilder>,

    pub base_metadata: Arc<RouteMetadata>,
    pub response_pipeline: Arc<ResponsePipeline>,
    pub plugins: Vec<Arc<dyn RidgePlugin>>,
    pub loaders: HashMap<String, Arc<dyn DataLoader>>,
    pub islands: HashMap<String, IslandConfig>,
    pub realtime: Vec<Arc<dyn RealtimeAdapter>>,
    pub render_modes: HashMap<RenderMode, crate::render::pipeline::ModeConfig>,
    pub edge_config: EdgeConfig,
    pub client_mirror: ClientMirrorConfig,
    pub diagnostics: DiagnosticsConfig,
    pub extensions: HashMap<String, Value>,
}