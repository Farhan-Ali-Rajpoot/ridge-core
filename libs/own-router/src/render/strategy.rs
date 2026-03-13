use axum::{
    body::to_bytes,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Response},
};
use serde_json::json;
use std::sync::Arc;
use tracing;

use crate::{
    metadata::core::RouteMetadata,           // your RouteMetadata
    router::context::UiResolutionContext,    // your context
    render::document::RootLayoutProps,       // your root props
};

// ==================== THE MAGIC TRAIT ====================
#[async_trait::async_trait]
pub trait ResponseStrategy: Send + Sync + 'static {
    /// Does this strategy want to handle the request?
    fn applies(&self, headers: &HeaderMap) -> bool;

    /// Turn the final inner HTML + metadata into the real response
    async fn respond(
        &self,
        inner_html: String,
        metadata: Arc<RouteMetadata>,           // we will use ResolvedMetadata later if needed
        ctx: Arc<UiResolutionContext>,
        original_response: Response,
    ) -> Response;
}

// ==================== PIPELINE (list of strategies) ====================
#[derive(Clone)]
pub struct ResponsePipeline {
    pub strategies: Vec<Arc<dyn ResponseStrategy>>,
}

impl ResponsePipeline {
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    pub fn with_strategy(mut self, strategy: impl ResponseStrategy + 'static) -> Self {
        self.strategies.insert(0, Arc::new(strategy)); // higher priority first
        self
    }
}

// ==================== YOUR CURRENT TWO MODES AS STRATEGIES ====================

// 1. Fragment Strategy (Own-Context header)
#[derive(Clone)]
pub struct FragmentStrategy;

#[async_trait::async_trait]
impl ResponseStrategy for FragmentStrategy {
    fn applies(&self, headers: &HeaderMap) -> bool {
        headers.contains_key("Own-Context")
    }

    async fn respond(
        &self,
        inner_html: String,
        _metadata: Arc<RouteMetadata>,
        ctx: Arc<UiResolutionContext>,
        _original: Response,
    ) -> Response {
        let payload = json!({
            "html": inner_html,
            "metadata": ctx.metadata.json,        // your existing ResolvedMetadata::json
            "stack_id": ctx.stack_id.as_ref(),
        });
        Json(payload).into_response()
    }
}

// 2. Full HTML Strategy (default fallback)
#[derive(Clone)]
pub struct FullHtmlStrategy;

#[async_trait::async_trait]
impl ResponseStrategy for FullHtmlStrategy {
    fn applies(&self, _headers: &HeaderMap) -> bool {
        true // fallback
    }

    async fn respond(
        &self,
        mut inner_html: String,
        metadata: Arc<RouteMetadata>,
        ctx: Arc<UiResolutionContext>,
        original_response: Response,
    ) -> Response {
        // === YOUR EXISTING LAYOUT WRAPPING LOGIC (moved here, unchanged) ===
        let client_stack = ""; // TODO: read from header later
        let client_parts: Vec<&str> = client_stack.split('/').filter(|s| !s.is_empty()).collect();
        let target_parts: Vec<&str> = ctx.stack_id.split('/').filter(|s| !s.is_empty()).collect();

        let common_count = client_parts
            .iter()
            .zip(&target_parts)
            .take_while(|(c, t)| c == t)
            .count();

        for i in (common_count..ctx.layout_stack.len()).rev() {
            let props = super::document::LayoutProps {  // your LayoutProps
                children: Some(std::borrow::Cow::Owned(inner_html)),
                class: None,
            };
            inner_html = (ctx.layout_stack[i].func)(props).await;
        }

        // === TERMINAL ROOT DOCUMENT (your DefaultDocumentLayout) ===
        let root_props = RootLayoutProps {
            children: std::borrow::Cow::Owned(inner_html),
            metadata: Some(std::borrow::Cow::Owned(metadata.render_html())), // ← your huge method!
            class: None,
        };

        let full_html = super::document::DefaultDocumentLayout(root_props); // or .await if async

        let mut res = (original_response.into_parts().0, Html(full_html)).into_response();
        if let Ok(h) = ctx.stack_id.parse() {
            res.headers_mut().insert("X-Response-Stack", h);
        }
        res
    }
}