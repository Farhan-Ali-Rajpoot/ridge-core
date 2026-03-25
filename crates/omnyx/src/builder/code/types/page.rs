use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::http::Method;
use serde_json::Value;

use crate::builder::code::CodeRouteBuilder;
use crate::core::router::registry::{RouteNode};
use crate::core::router::tree::Path;
use crate::core::router::handlers::{PageComponent, ErrorComponent};
use crate::core::router::logic::{DataLoader, Middleware, RouteMetadata};


pub struct PageDefinition {
    pub path: Path,
    pub handlers: HashMap<Method, Arc<dyn PageComponent>>,
    pub error_handlers: HashMap<Method, Arc<dyn ErrorComponent>>,
    pub metadata: RouteMetadata,
    pub children: Vec<RouteNode>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
    pub extensions: HashMap<String, Value>,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
}

impl PageDefinition {
    // Custom Methods
    // Usage: .method("FARHAN", handler)
    pub fn method<H: PageComponent + 'static>(mut self, verb: &str, handler: H) -> Self {
        let m = Method::from_bytes(verb.to_uppercase().as_bytes())
            .expect("Invalid HTTP method string");
            
        self.handlers.insert(m, Arc::new(handler));
        self
    }

    pub fn error_method<H: ErrorComponent + 'static>(mut self, verb: &str, handler: H) -> Self {
        let m = Method::from_bytes(verb.to_uppercase().as_bytes())
            .expect("Invalid HTTP method string");
            
        self.error_handlers.insert(m, Arc::new(handler));
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


    pub fn extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Page {
            path: self.path,
            handlers: self.handlers,
            error_handlers: self.error_handlers,
            metadata: self.metadata,
            children: self.children,
            loaders: self.loaders,
            extensions: self.extensions,
            middlewares: self.middlewares,
        };
        builder.roots.push(node);
    }
}
