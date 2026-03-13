use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::{RouteNode, PathSegment, RouteHandler, RouteAction};
use crate::render::{RenderMode, LayoutFn};
use crate::core::router::metadata::RouteMetadata;
use crate::loader::DataLoader;
use crate::builder::code::CodeRouteBuilder;



pub struct PageDefination {
    pub segment: PathSegment,
    pub handlers: HashMap<Method, Arc<dyn RouteHandler>>,
    pub loaders: Vec<Arc<dyn DataLoader>>,
    pub actions: Vec<Arc<dyn RouteAction>>,
    pub metadata: RouteMetadata,
    pub render_mode: Option<RenderMode>,
    pub children: Vec<RouteNode>,
    pub extensions: HashMap<String, Value>,
}

impl PageDefination {
    pub fn get<H: RouteHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.insert(Method::GET, Arc::new(handler));
        self
    }

    pub fn post<H: RouteHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.insert(Method::POST, Arc::new(handler));
        self
    }

    pub fn put<H: RouteHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.insert(Method::PUT, Arc::new(handler));
        self
    }

    pub fn delete<H: RouteHandler + 'static>(mut self, handler: H) -> Self {
        self.handlers.insert(Method::DELETE, Arc::new(handler));
        self
    }

    ///
    pub fn action<A: RouteAction + 'static>(mut self, action: A) -> Self {
        self.actions.push(Arc::new(action));
        self
    }

    pub fn loader<L: DataLoader + 'static>(mut self, loader: L) -> Self {
        self.loaders.push(Arc::new(loader));
        self
    }

    pub fn render_mode(mut self, mode: RenderMode) -> Self {
        self.render_mode = Some(mode);
        self
    }

    pub fn extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Route {
            segment: self.segment,
            handlers: self.handlers,
            metadata: self.metadata,
            children: self.children,
            loaders: self.loaders,
            actions: self.actions,
            render_mode: self.render_mode,
            extensions: self.extensions,
        };
        builder.roots.push(node);
    }
}
