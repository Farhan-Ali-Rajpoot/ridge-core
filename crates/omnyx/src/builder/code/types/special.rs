use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

use crate::builder::code::CodeRouteBuilder;
use crate::core::router::registry::{RouteNode, SpecialNodeKind};
use crate::core::router::handlers::{SpecialComponent};
use crate::core::router::logic::{DataLoader, Middleware};


pub struct SpecialDefinition {
    pub kind: SpecialNodeKind,
    pub component: Option<Arc<dyn SpecialComponent>>,
    pub children: Vec<RouteNode>,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
    pub extensions: HashMap<String, Value>,
}

impl SpecialDefinition {
    pub fn component<C: SpecialComponent + 'static>(mut self, c: C) -> Self {
        self.component = Some(Arc::new(c));
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

    pub fn  extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    
    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Special {
            kind: self.kind,
            component: self.component,
            children: self.children,
            middlewares: self.middlewares,
            loaders: self.loaders,
            extensions: self.extensions,
        };
        builder.roots.push(node);
    }
}