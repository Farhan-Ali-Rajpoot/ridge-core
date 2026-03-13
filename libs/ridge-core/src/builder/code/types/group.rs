use std::collections::HashMap;
use serde_json::Value;

use crate::builder::code::CodeRouteBuilder;
use crate::core::router::RouteNode;






pub struct GroupDefinition {
    pub id: String,
    pub children: Vec<RouteNode>,
    pub extensions: HashMap<String, Value>,
}

impl GroupDefinition {
    pub fn extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Group {
            id: self.id,
            children: self.children,
            extensions: self.extensions,
        };
        builder.roots.push(node);
    }
}





