use std::collections::HashMap;
use serde_json::Value;

use crate::core::router::PathSegment;
use crate::core::router::RouteNode;
use crate::core::router::SpecialNodeKind;
use crate::render::LayoutFn;
use crate::core::router::metadata::RouteMetadata;
use crate::builder::code::types::{
    layout::LayoutDefinition,
    api::ApiDefination,
    page::PageDefination,
    middleware::MiddlewareDefinition,
    extension::ExtensionDefinition,
    special::SpecialDefinition,
    group::GroupDefinition,
};





pub struct CodeRouteBuilder {
    pub roots: Vec<RouteNode>,
}

impl CodeRouteBuilder {
    pub fn new() -> Self {
        Self { roots: vec![] }
    }

    // pub fn route(&mut self, )

    pub fn page(&mut self, path: impl Into<String>) -> PageDefination {
        PageDefination {
            segment: PathSegment::parse_segment(&path.into()),
            handlers: HashMap::new(),
            loaders: vec![],
            actions: vec![],
            metadata: RouteMetadata::default(),
            render_mode: None,
            children: vec![],
            extensions: HashMap::new(),
        }
    }

    pub fn layout(&mut self, id: impl Into<String>) -> LayoutDefinition {
        LayoutDefinition {
            id: id.into(),
            component: None,
            metadata: RouteMetadata::default(),
            children: vec![],
            parallel_slots: HashMap::new(),
            extensions: HashMap::new(),
        }
    }

    pub fn group(&mut self, id: impl Into<String>) -> GroupDefinition {
        GroupDefinition {
            id: id.into(),
            children: vec![],
            extensions: HashMap::new(),
        }
    }

    pub fn middleware(&mut self) -> MiddlewareDefinition {
        MiddlewareDefinition {
            middlewares: vec![],
            children: vec![],
        }
    }

    pub fn special(&mut self, kind: SpecialNodeKind) -> SpecialDefinition {
        SpecialDefinition {
            kind,
            component: None,
            children: vec![],
        }
    }

    pub fn extension(&mut self, node_type: impl Into<String>) -> ExtensionDefinition {
        ExtensionDefinition {
            node_type: node_type.into(),
            data: Value::Null,
            children: vec![],
        }
    }


    // pub fn build(self) -> Vec<RouteNode> {
    //     self.roots
    // }
}

