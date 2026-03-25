use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::registry::RouteNode;
use crate::core::router::handlers::{LayoutComponent, ErrorComponent};
use crate::builder::code::CodeRouteBuilder;
use crate::core::router::logic::{RouteMetadata, Middleware, DataLoader};


pub struct LayoutDefinition {
    pub id: String,
    pub component: Option<Arc<dyn LayoutComponent>>,
    pub error_component: Option<Arc<dyn ErrorComponent>>,
    pub metadata: RouteMetadata,
    pub children: Vec<RouteNode>,
    pub slots: HashMap<String, Vec<RouteNode>>,
    pub extensions: HashMap<String, Value>,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
}

impl LayoutDefinition {
    pub fn component<F: LayoutComponent + 'static>(mut self, f: F) -> Self {
        self.component = Some(Arc::new(f));
        self
    }

    pub fn error_component<H: ErrorComponent + 'static>(mut self, f: H) -> Self {
        self.error_component = Some(Arc::new(f));
        self
    }

    pub fn middleware<M: Middleware + Send + Sync + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    pub fn  extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn parallel_slot(mut self, name: impl Into<String>, node: RouteNode) -> Self {
        self.slots
            .entry(name.into())
            .or_default()
            .push(node);
        self
    }

    pub fn loader<L: DataLoader + 'static>(mut self, loader: L) -> Self {
        self.loaders.push(Arc::new(loader));
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Layout {
            id: self.id,
            component: self.component,
            error_component: self.error_component,
            metadata: self.metadata,
            children: self.children,
            slots: self.slots,
            extensions: self.extensions,
            middlewares: self.middlewares,
            loaders: self.loaders,
        };
        builder.roots.push(node);
    }
}

