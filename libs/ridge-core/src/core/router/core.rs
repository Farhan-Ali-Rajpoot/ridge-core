use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use async_trait::async_trait;
use axum::http::{self, Method};

use crate::core::router::metadata::RouteMetadata;
use crate::render::layout::LayoutFn;
use crate::middleware::RidgeMiddleware;
use crate::loader::DataLoader;
use crate::render::mode::RenderMode;
use crate::error::RouteError;
use crate::builder::route_tree_builder::RouteTreeBuilderInfo;

use crate::core::router::RouteNode;
use crate::core::router::{RouteMatcher, MatchedRoute};


#[derive(Clone)]
pub struct RouteTree {
    pub roots: Vec<RouteNode>,                              // new()
    pub matcher: Arc<dyn RouteMatcher + Send + Sync>,       // new()
    pub builder_info: RouteTreeBuilderInfo,                 // new()
    pub extensions: HashMap<String, Value>,                 // build_cache()
    pub by_id: HashMap<String, RouteNode>,                  // build_cache()
    pub by_pattern: HashMap<String, RouteNode>,             // build_cache()
    pub layout_chains: HashMap<String, Vec<String>>,        // build_cache()
    pub merged_metadata: HashMap<String, RouteMetadata>,    // build_cache()
}

impl RouteTree {
    pub fn new(
        roots: Vec<RouteNode>,
        matcher: Arc<dyn RouteMatcher + Send + Sync>,
        builder_info: RouteTreeBuilderInfo,
    ) -> Self {
        Self {
            roots,
            by_id: HashMap::new(),
            by_pattern: HashMap::new(),
            matcher,
            layout_chains: HashMap::new(),
            merged_metadata: HashMap::new(),
            builder_info,
            extensions: HashMap::new(),
        }
    }


    pub fn build_caches(&mut self) -> Result<(), RouteError> {
        fn traverse(
            node: &RouteNode,               
            current_id: &str,
            current_pattern: &str,
            layout_stack: &mut Vec<String>,
            current_metadata: &RouteMetadata,   
            by_id: &mut HashMap<String, RouteNode>,
            by_pattern: &mut HashMap<String, RouteNode>,
            layout_chains: &mut HashMap<String, Vec<String>>,
            merged_metadata: &mut HashMap<String, RouteMetadata>,
        ) -> Result<(), RouteError> {
            match node {
                RouteNode::Route { 
                    segment, 
                    metadata, 
                    children, 
                    .. 
                } => {
                    let segment_str = segment.to_matchit_pattern();

                    let new_pattern = if current_pattern.is_empty() {
                        segment_str.clone()
                    } else {
                        format!("{}/{}", current_pattern, segment_str)
                    };

                    let route_id = if current_id.is_empty() {
                        new_pattern.clone()
                    } else {
                        format!("{}/{}", current_id, segment_str)
                    };

                    // === CACHES ===
                    by_id.insert(route_id.clone(), node.clone());
                    by_pattern.insert(new_pattern.clone(), node.clone());

                    // === METADATA INHERITANCE (using your method) ===
                    let mut merged = metadata.clone();
                    merged.inherit_from(current_metadata);           // ← your method

                    merged_metadata.insert(route_id.clone(), merged.clone());

                    // === LAYOUT CHAIN ===
                    layout_chains.insert(route_id.clone(), layout_stack.clone());

                    // Recurse
                    for child in children {
                        traverse(child, &route_id, &new_pattern, layout_stack, &merged, 
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }
                }

                RouteNode::Layout { 
                    id, 
                    metadata, 
                    children, 
                    parallel_slots, 
                    .. 
                } => {
                    layout_stack.push(id.clone());

                    let mut merged = metadata.clone();
                    merged.inherit_from(current_metadata);

                    // Normal children
                    for child in children {
                        traverse(child, current_id, current_pattern, layout_stack, &merged, 
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }

                    // Parallel slots (@sidebar, @modal, etc.)
                    for slots in parallel_slots.values() {
                        for slot in slots {
                            traverse(slot, current_id, current_pattern, layout_stack, &merged, 
                                     by_id, by_pattern, layout_chains, merged_metadata)?;
                        }
                    }

                    layout_stack.pop();
                }

                RouteNode::Group { id, children, .. } => {
                    let new_id = if current_id.is_empty() {
                        id.clone()
                    } else {
                        format!("{}/{}", current_id, id)
                    };

                    for child in children {
                        traverse(child, &new_id, current_pattern, layout_stack, current_metadata,
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }
                }

                RouteNode::MiddlewareBoundary { children, .. } => {
                    for child in children {
                        traverse(child, current_id, current_pattern, layout_stack, current_metadata,
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }
                }

                RouteNode::Special { kind, children, .. } => {
                    let kind_str = format!("{:?}", kind).to_lowercase();
                    let new_pattern = if current_pattern.is_empty() {
                        format!("/_{}", kind_str)
                    } else {
                        format!("{}/_{}", current_pattern, kind_str)
                    };

                    for child in children {
                        traverse(child, current_id, &new_pattern, layout_stack, current_metadata,
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }
                }

                RouteNode::Extension { children, .. } => {
                    for child in children {
                        traverse(child, current_id, current_pattern, layout_stack, current_metadata,
                                 by_id, by_pattern, layout_chains, merged_metadata)?;
                    }
                }
            }
            Ok(())
        }

        let mut layout_stack = Vec::new();
        let default_metadata = RouteMetadata::default();

        for root in &self.roots {
            traverse(
                root,
                "",
                "",
                &mut layout_stack,
                &default_metadata,
                &mut self.by_id,
                &mut self.by_pattern,
                &mut self.layout_chains,
                &mut self.merged_metadata,
            )?;
        }

        Ok(())
    }


}