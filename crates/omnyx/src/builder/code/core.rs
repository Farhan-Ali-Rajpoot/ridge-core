use std::collections::HashMap;
use serde_json::Value;

use crate::core::router::Path;
use crate::core::router::RouteNode;
use crate::core::router::SpecialNodeKind;
use crate::core::router::metadata::RouteMetadata;
use crate::builder::code::types::{LayoutDefinition, ApiDefinition, PageDefinition, SpecialDefinition, GroupDefinition};




pub struct CodeRouteBuilder {
    pub roots: Vec<RouteNode>,
}

impl CodeRouteBuilder {
    pub fn new() -> Self {
        Self { roots: vec![] }
    }

    pub fn page(&mut self, path: impl Into<String>) -> PageDefinition {
        PageDefinition {
            path: Path::from_str(&path.into()),
            handlers: HashMap::new(),
            error_handlers: HashMap::new(),
            loaders: vec![],
            middlewares: vec![],
            metadata: RouteMetadata::default(),
            children: vec![],
            extensions: HashMap::new(),
        }
    }

    pub fn api(&mut self, path: impl Into<String>) -> ApiDefinition {
        ApiDefinition {
            path: Path::from_str(&path.into()),
            handlers: HashMap::new(),
            children: vec![],
            middlewares: vec![],
            extensions: HashMap::new(),
            loaders: vec![],
        }
    }

    pub fn layout(&mut self, id: impl Into<String>) -> LayoutDefinition {
        LayoutDefinition {
            id: id.into(),
            component: None,
            error_component: None,
            metadata: RouteMetadata::default(),
            children: vec![],
            slots: HashMap::new(),
            extensions: HashMap::new(),
            middlewares: vec![],
            loaders: vec![],
        }
    }

    pub fn group(&mut self, id: impl Into<String>) -> GroupDefinition {
        GroupDefinition {
            id: id.into(),
            children: vec![],
            extensions: HashMap::new(),
            middlewares: vec![],
            loaders: vec![],
        }
    }

    pub fn special(&mut self, kind: SpecialNodeKind) -> SpecialDefinition {
        SpecialDefinition {
            kind,
            component: None,
            children: vec![],
            middlewares: vec![],
            loaders: vec![],
            extensions: HashMap::new(),
        }
    }



    // pub fn build(self) -> Vec<RouteNode> {
    //     self.roots
    // }
}

