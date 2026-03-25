use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

use crate::builder::code::CodeRouteBuilder;
use crate::core::router::registry::RouteNode;
use crate::core::router::logic::{DataLoader, Middleware};





pub struct GroupDefinition {
    pub id: String,
    pub children: Vec<RouteNode>,
    pub extensions: HashMap<String, Value>,
    pub middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
}

impl GroupDefinition {
    pub fn extension(mut self, key: impl Into<String>, value: Value) -> Self {
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
        let node = RouteNode::Group {
            id: self.id,
            children: self.children,
            extensions: self.extensions,
            middlewares: self.middlewares,
            loaders: self.loaders,
        };
        builder.roots.push(node);
    }
}





