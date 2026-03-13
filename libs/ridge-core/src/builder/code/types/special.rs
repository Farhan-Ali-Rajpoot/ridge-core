use std::sync::Arc;


use crate::core::router::RouteNode ;
use crate::core::router::SpecialNodeKind;
use crate::core::router::SpecialComponent;
use crate::builder::code::CodeRouteBuilder;




pub struct SpecialDefinition {
    pub kind: SpecialNodeKind,
    pub component: Option<Arc<dyn SpecialComponent>>,
    pub children: Vec<RouteNode>,
}

impl SpecialDefinition {
    pub fn component<C: SpecialComponent + 'static>(mut self, c: C) -> Self {
        self.component = Some(Arc::new(c));
        self
    }

    pub fn finish(mut self, builder: &mut CodeRouteBuilder) {
        let node = RouteNode::Special {
            kind: self.kind,
            component: self.component,
            children: self.children,
        };
        builder.roots.push(node);
    }
}