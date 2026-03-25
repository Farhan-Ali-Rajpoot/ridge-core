use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;
use axum::http::Method;
use matchit::{Match, Router as MatchitRouter, InsertError};

use crate::core::router::registry::{RouteNode, SpecialNodeKind};
use crate::error::RouteError;
use crate::core::router::logic::{DataLoader, RouteMetadata, Middleware};


#[async_trait]
pub trait RouteMatcher: Send + Sync + 'static {
    fn match_route(&self, path: &str, method: Method) -> Option<MatchedRoute>;
    fn register(&mut self, node: &RouteNode) -> Result<(), RouteError>;
}

#[derive(Clone, Debug)]
pub struct MatchedRoute {
    pub node: RouteNode,
    pub params: HashMap<String, String>,
    pub matched_pattern: String,
    
}

#[derive(Clone, Debug)]
pub struct MatchitMatcher {
    router: Arc<MatchitRouter<RouteEntry>>,
}

#[derive(Clone, Debug)]
pub struct RouteEntry {
    pub node: Arc<RouteNode>,
    pub methods: HashSet<Method>,
    pub matched_pattern: String,

    // pub layout_chain: Arc<[Arc<RouteNode>]>, 
    // pub middleware_chain: Arc<[Arc<dyn Middleware>]>,
    // pub loader_chain: Arc<[Arc<dyn DataLoader>]>,

    // // 3. The Pre-computed State
    // pub merged_metadata: Arc<RouteMetadata>,
    // pub merged_extensions: Arc<HashMap<String, Value>>,
    // pub matched_pattern: String,
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
            RouteNode::Page { path, handlers, children, .. } => {
                let path_pattern = path.to_matchit_pattern()
                    .replace("[", "{")
                    .replace("[[", "{")
                    .replace("]","}")
                    .replace("]]","}")
                    .replace("...","*");

                let full_pattern = if parent_pattern.is_empty() || parent_pattern == "/" {
                    // If we are at the root, ensure we start with a single "/"
                    if path_pattern.starts_with('/') { path_pattern } else { format!("/{}", path_pattern) }
                } else {
                    // If nesting, we trim slashes from the join point to prevent "parent//child"
                    format!("{}/{}", parent_pattern.trim_end_matches('/'), path_pattern.trim_start_matches('/'))
                };

                let methods: HashSet<Method> = handlers.keys().cloned().collect();

                if methods.is_empty() {
                    return Err(RouteError::MissingHandler("Page has no handlers".into()));
                }

                let entry = RouteEntry {
                    node: Arc::new(node.clone()),
                    methods,
                    matched_pattern: full_pattern.clone(),
                };

                router.insert(&full_pattern, entry)
                    .map_err(|e| RouteError::Conflict(e.to_string()))?;
                // Recusive
                for child in children {
                    self.register_recursive(child, router, full_pattern.clone())?;
                }
            }

            RouteNode::Api { path, handlers, children, .. } => {
                let path_pattern = path.to_matchit_pattern()
                    .replace("[", "{")
                    .replace("[[", "{")
                    .replace("]","}")
                    .replace("]]","}")
                    .replace("...","*");

                let full_pattern = if parent_pattern.is_empty() || parent_pattern == "/" {
                    if path_pattern.starts_with("/") { path_pattern } else { format!("/{}",path_pattern)}
                } else {
                    format!("{}/{}", parent_pattern.trim_end_matches("/"), path_pattern.trim_start_matches("/"))
                };

                let methods: HashSet<Method> = handlers.keys().cloned().collect();

                if methods.is_empty() {
                    return Err(RouteError::MissingHandler("API route has no handlers".into()));
                }

                let entry = RouteEntry {
                    node: Arc::new(node.clone()),
                    methods,
                    matched_pattern: full_pattern.clone(),
                };

                router.insert(&full_pattern, entry)
                    .map_err(|e| RouteError::Conflict(e.to_string()))?;

                // Recurse 
                for child in children {
                    self.register_recursive(child, router, full_pattern.clone())?;
                }
            }

            RouteNode::Layout { children, slots, .. } => {
                for child in children {
                    self.register_recursive(child, router, parent_pattern.clone())?;
                }
                for slots in slots.values() {
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

            RouteNode::Special { kind, children, .. } => {
                let kind_str = format!("{:?}", kind).to_lowercase();
                let new_pattern = if parent_pattern.is_empty() {
                    format!("/_{}", kind_str)
                } else {
                    format!("{}/_{}", parent_pattern.clone(), kind_str)
                };

                let entry = RouteEntry {
                    node: Arc::new(node.clone()),
                    methods: HashSet::new(), 
                    matched_pattern: new_pattern.clone(),
                };
                router.insert(&new_pattern, entry).ok(); 

                for child in children {
                    self.register_recursive(child, router, new_pattern.clone())?;
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl RouteMatcher for MatchitMatcher {
    fn match_route(&self, path: &str, method: Method) -> Option<MatchedRoute> {

        let normalized_path = if path.starts_with("/") {
            path
        }else {
            &format!("/{}", path)
        };

        let matched = match self.router.at(normalized_path) {
            Ok(m) => m,
            Err(_) => return None,
        };

        if !matched.value.methods.contains(&method) {
            return None; 
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

    fn register(&mut self, node: &RouteNode) -> Result<(), RouteError> {
        let mut router = Arc::try_unwrap(self.router.clone())
            .unwrap_or_else(|arc| (*arc).clone());

        self.register_recursive(node, &mut router, String::new())?;

        self.router = Arc::new(router);
        Ok(())
    }
}