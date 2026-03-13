use serde_json::Value;
use axum::{
    body::Body,
    http::{self, Method, Request, request::Parts, Extensions, HeaderMap, HeaderValue, header::HeaderName},
};

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::router::RouteNode;


// #[derive(Debug)]
pub struct RequestContext {
    pub req: Request<Body>,

    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,

    pub extensions: Extensions,

    pub data: HashMap<String, Value>,

    pub matched_node: Option<Arc<RouteNode>>,

    pub response_headers: HeaderMap,
    pub response_extensions: Extensions,

    pub loader_cache: HashMap<String, Value>,
}


impl RequestContext {
    pub fn new(
        req: Request<Body>,
        params: HashMap<String, String>,
        matched_node: Option<Arc<RouteNode>>,
    ) -> Self {
        let (parts, body) = req.into_parts();
        let mut query = HashMap::new();
        if let Some(q) = parts.uri.query() {
            for (k, v) in q.split("&").filter_map(|s| s.split_once("=")) {
                query.insert(k.into(), v.into());
            }
        }

        Self {
            req: Request::from_parts(parts, body),
            params,
            query,
            extensions: Extensions::new(),
            data: HashMap::new(),
            matched_node,
            response_headers: HeaderMap::new(),
            response_extensions: Extensions::new(),
            loader_cache: HashMap::new(),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.extensions.get::<T>()
    }

    pub fn insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) {
        self.extensions.insert(value);
    }

    pub fn set_loader_data(&mut self, loader_id: impl Into<String>, data: Value) {
        self.loader_cache.insert(loader_id.into(), data);
    }

    pub fn set_response_header(&mut self, key: HeaderName, value: HeaderValue) {
        self.response_headers.insert(key, value);
    }
}