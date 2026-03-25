use std::collections::HashMap;
use std::sync::Arc;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::tree::Path;
use crate::core::router::handlers::ApiHandler;
use crate::core::router::registry::RouteNode;
use crate::builder::CodeRouteBuilder;
use crate::core::router::logic::{DataLoader, Middleware};


pub struct ApiDefinition {
    pub path: Path,
    pub handlers: HashMap<Method, Arc<dyn ApiHandler>>,
    pub children: Vec<RouteNode>,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    pub extensions: HashMap<String, Value>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
    
}

impl ApiDefinition {
    // Custom Methods
    // Usage: .method("FARHAN", handler)
    pub fn method<H: ApiHandler + 'static>(mut self, verb: &str, handler: H) -> Self {
        let m = Method::from_bytes(verb.to_uppercase().as_bytes())
            .expect("Invalid HTTP method string");
            
        self.handlers.insert(m, Arc::new(handler));
        self
    }

    pub fn  extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn middleware<M: Middleware + Send + Sync + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    pub fn loader<L: DataLoader + 'static>(mut self, loader: L) -> Self {
        self.loaders.push(Arc::new(loader));
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Api {
            path: self.path,
            handlers: self.handlers,
            children: self.children,
            middlewares: self.middlewares,
            extensions: self.extensions,
            loaders: self.loaders,
        };
        builder.roots.push(node);
    }
}