use std::collections::HashMap;
use std::sync::Arc;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::PathSegment;
use crate::core::router::RouteNode;
use crate::middleware::RidgeMiddleware;


pub struct ApiDefination {
    segment: PathSegment,
    handlers: HashMap<Method, Arc<dyn ApiHandler>>,
    children: Vec<RouteNode>,
    middlewares: Vec<Arc<dyn RidgeMiddleware + Send + Sync>>,
    extensions: HashMap<String, Value>,
}