use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::RouteMatcher;
use crate::core::router::RouteTree;
use crate::error::RouteError;
use crate::render::layout::LayoutFn;
use crate::core::router::MatchitMatcher;
use crate::plugin::RidgePlugin;
use crate::loader::DataLoader;
use crate::island::IslandConfig;
use crate::realtime::RealtimeAdapter;
use crate::builder::code::core::CodeRouteBuilder;
use crate::builder::route_tree_builder::{RouteTreeBuilder, RouteTreeBuilderInfo};
use crate::render::RenderMode;
use crate::render::ModeConfig;
use crate::edge::EdgeConfig;
use crate::client::ClientMirrorConfig;

#[derive(Default)]
pub struct RidgeBuilder {
    code_builders: Vec<Arc<CodeRouteBuilder>>,

    plugins: Vec<Arc<dyn RidgePlugin>>,
    loaders: HashMap<String, Arc<dyn DataLoader>>,
    islands: HashMap<String, IslandConfig>,
    realtime_adapters: Vec<Arc<dyn RealtimeAdapter>>,
    render_modes: HashMap<RenderMode, ModeConfig>,
    edge_config: EdgeConfig,
    client_mirror: ClientMirrorConfig,
    diagnostics: DiagnosticsConfig,
    extensions: HashMap<String, Value>,
}

impl RidgeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut CodeRouteBuilder) + Send + Sync + 'static,
    {
        let mut cb = CodeRouteBuilder::new();
        f(&mut cb);
        self.code_builders.push(Arc::new(cb));
        self
    }
}

#[async_trait]
impl RouteTreeBuilder for RidgeBuilder {
    async fn build(&self) -> Result<RouteTree, RouteError> {
        let mut all_roots = vec![];

        for cb in &self.code_builders {
            all_roots.extend(cb.roots.clone());
        }

        let matcher: Arc<dyn RouteMatcher + Send + Sync> =
            Arc::new(MatchitMatcher::new()); 

        let builder_info = RouteTreeBuilderInfo::CodeDefined {
            code_blocks_count: 90,
            built_at: std::time::SystemTime::now(),
            description: Some("Builded!".into()),
        };

        let mut tree = RouteTree::new(all_roots, matcher, builder_info);
        tree.build_caches()?;
        Ok(tree)
    }
}




