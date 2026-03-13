use std::sync::Arc;

use crate::middleware::RidgeMiddleware;
use crate::core::router::RouteNode;
use crate::builder::code::CodeRouteBuilder;

pub struct MiddlewareDefinition {
    pub middlewares: Vec<Arc<dyn RidgeMiddleware + Send + Sync>>,
    pub children: Vec<RouteNode>,
}

impl MiddlewareDefinition {
    pub fn with<M: RidgeMiddleware + Send + Sync + 'static>(mut self, mw: M) -> Self {
        self.middlewares.push(Arc::new(mw));
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::MiddlewareBoundary {
            middlewares: self.middlewares,
            children: self.children,
        };
        builder.roots.push(node);
    }
}