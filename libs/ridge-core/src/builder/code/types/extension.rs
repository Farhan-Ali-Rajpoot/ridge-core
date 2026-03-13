use serde_json::Value;


use crate::core::router::RouteNode;
use crate::builder::code::CodeRouteBuilder;




pub struct ExtensionDefinition {
    pub node_type: String,
    pub data: Value,
    pub children: Vec<RouteNode>,
}

impl ExtensionDefinition {
    pub fn data(mut self, data: Value) -> Self {
        self.data = data;
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Extension {
            node_type: self.node_type,
            data: self.data,
            children: self.children,
        };
        builder.roots.push(node);
    }
}