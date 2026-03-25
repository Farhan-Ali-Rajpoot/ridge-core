use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use axum::http::{HeaderMap, Request, StatusCode};
use axum_extra::extract::CookieJar;
use serde_json::Value;
use bytes::Bytes;

use crate::core::router::tree::RouteEntry;
use crate::core::router::handlers::LayoutProps;
use crate::core::router::logic::RouteMetadata;
use crate::core::router::io::{RequestContext, ResponseContext};

#[derive(Clone, Default)]
pub struct FrameworkContext {
    pub request: Arc<RequestContext>,
    pub response: Arc<ResponseContext>,

    pub extensions: Arc<HashMap<String, Value>>,
    pub loader_cache: Arc<HashMap<String, Value>>,
    pub metadata: Arc<RouteMetadata>,

    pub matched_node: Option<Arc<RouteEntry>>,
    pub layout_props: Option<Arc<LayoutProps>>,
    pub layer: usize,
}


impl FrameworkContext {
    pub fn new(
        request: RequestContext,
        matched_node: Option<Arc<RouteEntry>>,
        extensions: Arc<HashMap<String, Value>>,
        metadata: Arc<RouteMetadata>,
    ) -> Self {
        Self {
            request: Arc::new(request),
            response: Arc::new(ResponseContext::default()),
            extensions,
            metadata,
            loader_cache: Arc::new(HashMap::new()),
            matched_node,
            layout_props: None,
            layer: 0,
        }
    }

    #[inline(always)]
    pub fn response_mut(&mut self) -> &mut ResponseContext {
        Arc::make_mut(&mut self.response)
    }

    #[inline(always)]
    pub fn response(&self) -> &ResponseContext {
        self.response.as_ref()
    }

    #[inline(always)]
    pub fn loader_cache_mut(&mut self) -> &mut HashMap<String, Value> {
        Arc::make_mut(&mut self.loader_cache)  
    }

    #[inline(always)]
    pub fn loader_cache(&self) -> &HashMap<String, Value> {
        &self.loader_cache
    }

    #[inline(always)]
    pub fn extensions_mut(&mut self) -> &mut HashMap<String, Value> {
        Arc::make_mut(&mut self.extensions)
    }

    pub fn clone_shallow(&self) -> Self {
        self.clone()
    }
}