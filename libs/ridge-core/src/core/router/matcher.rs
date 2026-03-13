use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;
use axum::http::Method;
use matchit::{Match, Router as MatchitRouter, InsertError};

use crate::core::router::{RouteNode, PathSegment, SpecialNodeKind};
use crate::error::{RouteError, RidgeError};

#[async_trait]
pub trait RouteMatcher: Send + Sync + 'static {
    fn match_route(&self, path: &str, method: Method) -> Option<MatchedRoute>;
    fn register(&mut self, node: &RouteNode) -> Result<(), RidgeError>;
}

#[derive(Clone)]
pub struct MatchedRoute {
    pub node: RouteNode,
    pub params: HashMap<String, String>,
    pub matched_pattern: String,
}

#[derive(Clone)]
pub struct MatchitMatcher {
    router: Arc<MatchitRouter<RouteEntry>>,
}

#[derive(Clone)]
struct RouteEntry {
    node: Arc<RouteNode>,
    methods: HashSet<Method>, 
    matched_pattern: String,  
}


impl MatchitMatcher {

    pub fn new() -> Self {
        Self {
            router: Arc::new(MatchitRouter::new()),
        }
    }

    fn register_recursive(
        &self,
        node: &RouteNode,
        router: &mut MatchitRouter<RouteEntry>,
        parent_pattern: String,
    ) -> Result<(), RouteError> {
        match node {
            RouteNode::Route { segment, handlers, children, .. } => {
                let segment_pattern = segment.to_matchit_pattern();

                let full_pattern = if parent_pattern.is_empty() {
                    segment_pattern
                } else {
                    format!("{}/{}", parent_pattern, segment_pattern)
                };

                let methods: HashSet<Method> = handlers.keys().cloned().collect();

                if methods.is_empty() {
                    return Err(RouteError::MissingHandler("Route has no handlers".into()).into());
                }

                let entry = RouteEntry {
                    node: Arc::new(node.clone()),
                    methods,
                    matched_pattern: full_pattern.clone(),
                };

                router.insert(&full_pattern, entry)
                    .map_err(|e| RouteError::Conflict(e.to_string()).into())?;

                for child in children {
                    self.register_recursive(child, router, full_pattern.clone())?;
                }
            }

            RouteNode::Layout { children, parallel_slots, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
                for slots in parallel_slots.values() {
                    for slot in slots {
                        self.register_recursive(slot, router, parent_pattern.clone())?;
                    }
                }
            }

            RouteNode::Group { children, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
            }

            RouteNode::MiddlewareBoundary { children, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
            }

            RouteNode::Special { kind, children, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
                // Future: auto-register special paths
            }

            RouteNode::Extension { node_type, data, children, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
                // Future: plugin hook here
            }
        }

        Ok(())
    }
}

#[async_trait]
impl RouteMatcher for MatchitMatcher {
    fn match_route(&self, path: &str, method: Method) -> Option<MatchedRoute> {
        let matched: Match<'_, '_, &RouteEntry> = match self.router.at(path) {
            Ok(m) => m,
            Err(_) => return None,
        };

        if !matched.value.methods.contains(&method) {
            return None;  // 405 Method Not Allowed later in pipeline
        }

        let params: HashMap<String, String> = matched
            .params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        Some(MatchedRoute {
            node: (*matched.value.node).clone(),
            params,
            matched_pattern: matched.value.matched_pattern.clone(),
        })
    }

    fn register(&mut self, node: &RouteNode) -> Result<(), RidgeError> {
        let mut router = Arc::try_unwrap(self.router.clone())
            .unwrap_or_else(|arc| (*arc).clone());

        self.register_recursive(node, &mut router, String::new())?;

        self.router = Arc::new(router);
        Ok(())
    }
}
