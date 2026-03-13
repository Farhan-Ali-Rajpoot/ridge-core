use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use axum::http::Method;
use serde_json::Value;

use crate::core::router::{RouteNode};
use crate::render::layout::LayoutFn;
use crate::core::router::metadata::RouteMetadata;
use crate::builder::code::CodeRouteBuilder;




pub struct LayoutDefinition {
    pub id: String,
    pub component: Option<Arc<dyn LayoutFn>>,
    pub metadata: RouteMetadata,
    pub children: Vec<RouteNode>,
    pub parallel_slots: HashMap<String, Vec<RouteNode>>,
    pub extensions: HashMap<String, Value>,
}

impl LayoutDefinition {
    pub fn component<F: LayoutFn + 'static>(mut self, f: F) -> Self {
        self.component = Some(Arc::new(f));
        self
    }

    pub fn parallel_slot(mut self, name: impl Into<String>, node: RouteNode) -> Self {
        self.parallel_slots
            .entry(name.into())
            .or_default()
            .push(node);
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Layout {
            id: self.id,
            component: self.component,
            metadata: self.metadata,
            children: self.children,
            parallel_slots: self.parallel_slots,
            extensions: self.extensions,
        };
        builder.roots.push(node);
    }
}

