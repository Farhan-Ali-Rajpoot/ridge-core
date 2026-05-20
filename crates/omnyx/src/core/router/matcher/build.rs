use std::sync::Arc;

use crate::collections::LinearMap;
use crate::core::ParallelRouteMatcher;
use crate::core::router::logic::{Middleware, RouteMetadata};
use crate::core::router::registry::ParallelRouteNode;
use crate::core::router::registry::RouteNode;
use crate::error::RouteError;

use super::{
    RouteMatcher,
    core::{ApiEndpoint, Layout, PageEndpoint, ParallelRoute, RouteEntry, RouteKind},
};

#[derive(Clone)]
pub struct RouteBakeContext {
    pub path: String,                 // accumulated URL path prefix
    pub layouts: Vec<Arc<Layout>>,   // layout chain
    pub middlewares: Vec<Arc<dyn Middleware>>,
    pub metadata: RouteMetadata,
    pub node_ids: Vec<String>,        // collect node IDs of layouts and page
}

#[derive(Clone)]
pub struct ParallelRouteBakeContext {
    pub path: String,            // absolute path prefix from parent layout
    pub slot_name: String,
    pub layouts: Vec<Arc<Layout>>,
    pub prefix_id: String,       // ID of the parent slot container (or layout for nested slots)
}

impl Default for RouteBakeContext {
    fn default() -> Self {
        Self {
            path: "/".into(),
            layouts: Vec::new(),
            middlewares: Vec::new(),
            metadata: RouteMetadata::default(),
            node_ids: Vec::new(),
        }
    }
}

impl RouteMatcher {
    pub fn build_with_nodes(root_nodes: Vec<RouteNode>) -> Result<Self, RouteError> {
        let mut router_matcher = RouteMatcher::new();
        for node in root_nodes {
            let ctx = RouteBakeContext::default();
            Self::bake_route_recursive(node, &mut router_matcher, ctx)?;
        }
        Ok(router_matcher)
    }

    fn bake_route_recursive(
        node: RouteNode,
        router: &mut RouteMatcher,
        ctx: RouteBakeContext,
    ) -> Result<(), RouteError> {
        match node {
            RouteNode::Page {
                path,
                controllers,
                error_controller,
                loader_controller,
                metadata,
                children,
                middlewares,
            } => {
                let mut current_ctx = ctx.clone();
                current_ctx.path = join_paths(&ctx.path, &path.to_matchit_pattern());
                current_ctx.middlewares.extend(middlewares);
                if let Some(meta) = metadata {
                    current_ctx.metadata.update_from_child(&meta);
                }

                if !controllers.is_empty() {
                    // Parent layout ID = the last layout in the chain
                    let parent_layout_id = if let Some(layout) = current_ctx.layouts.last() {
                        layout.node_id.clone()
                    } else {
                        "L_fallback".to_string()
                    };
                    let page_node_id = format!("{}:P", parent_layout_id);

                    // Collect all node IDs: layouts (in order) + page node ID
                    let mut node_ids = Vec::new();
                    for layout in &current_ctx.layouts {
                        node_ids.push(layout.node_id.clone());
                    }
                    node_ids.push(page_node_id.clone());

                    let page_endpoint = PageEndpoint {
                        controllers: controllers.clone(),
                        loader_controller: loader_controller.clone(),
                        error_controller: error_controller.clone(),
                        layouts: current_ctx.layouts.clone(),
                        metadata: current_ctx.metadata.clone(),
                        node_id: page_node_id.clone(),
                    };

                    let entry = RouteEntry {
                        matched_pattern: current_ctx.path.clone(),
                        middlewares: current_ctx.middlewares.clone(),
                        kind: RouteKind::Page(page_endpoint),
                        node_ids: node_ids.clone(),
                    };
                    router.resolve(&current_ctx.path, entry)?;

                    // ========== FIX: REMOVED the faulty optional‑catch‑all expansion ==========
                    // The pattern `/{*param}` already matches both the base path and any extra segments,
                    // so the extra registration is unnecessary and would cause a duplicate route error.
                }

                for child in children {
                    Self::bake_route_recursive(child, router, current_ctx.clone())?;
                }
            }

            RouteNode::Api {
                path,
                controllers,
                children,
                middlewares,
            } => {
                let mut current_ctx = ctx.clone();
                current_ctx.path = join_paths(&ctx.path, &path.to_matchit_pattern());
                current_ctx.middlewares.extend(middlewares);

                if !controllers.is_empty() {
                    let api_endpoint = ApiEndpoint { controllers };
                    let entry = RouteEntry {
                        matched_pattern: current_ctx.path.clone(),
                        middlewares: current_ctx.middlewares.clone(),
                        kind: RouteKind::Api(api_endpoint),
                        node_ids: vec![],
                    };
                    router.resolve(&current_ctx.path, entry)?;
                }

                for child in children {
                    Self::bake_route_recursive(child, router, current_ctx.clone())?;
                }
            }

            RouteNode::Layout {
                id,
                controller,
                error_controller,
                loader_controller,
                metadata,
                children,
                parallel_routes,
                middlewares,
            } => {
                let mut current_ctx = ctx.clone();
                current_ctx.middlewares.extend(middlewares);
                if let Some(meta) = metadata {
                    current_ctx.metadata.update_from_child(&meta);
                }

                // Compute layout ID
                let node_id = if current_ctx.path == "/" {
                    format!("L_{}", id)
                } else {
                    let norm = normalize_to_segment(&current_ctx.path);
                    format!("L_{}_{}", norm, id)
                };

                let mut current_layout = Layout {
                    base_path: current_ctx.path.clone(),
                    controller,
                    error_controller,
                    loader_controller,
                    parallel_routers: LinearMap::new(),
                    node_id: node_id.clone(),
                };

                // Process parallel slots
                for (slot_name, nodes) in parallel_routes {
                    let slot_container_id = format!("{}:S_{}", node_id, slot_name);
                    let mut slot_matcher = ParallelRouteMatcher::new();
                    for node in nodes {
                        bake_parallel_route_recursive(node, &mut slot_matcher, ParallelRouteBakeContext {
                            slot_name: slot_name.clone(),
                            path: current_ctx.path.clone(),
                            layouts: Vec::new(),
                            prefix_id: slot_container_id.clone(),
                        })?;
                    }
                    current_layout.parallel_routers.insert(slot_name, Arc::new(slot_matcher));
                }

                current_ctx.layouts.push(Arc::new(current_layout));
                current_ctx.node_ids.push(node_id.clone());

                for child in children {
                    Self::bake_route_recursive(child, router, current_ctx.clone())?;
                }
            }

            RouteNode::Group {
                id: _,
                children,
                metadata,
                middlewares,
            } => {
                let mut current_ctx = ctx.clone();
                current_ctx.middlewares.extend(middlewares);
                if let Some(meta) = metadata {
                    current_ctx.metadata.update_from_child(&meta);
                }
                for child in children {
                    Self::bake_route_recursive(child, router, current_ctx.clone())?;
                }
            }
        }
        Ok(())
    }
}

/// Bakes a parallel route tree into a `ParallelRouteMatcher` for a single slot.
fn bake_parallel_route_recursive(
    node: ParallelRouteNode,
    matcher: &mut ParallelRouteMatcher,
    ctx: ParallelRouteBakeContext,
) -> Result<(), RouteError> {
    match node {
        ParallelRouteNode::Page {
            path,
            controller,
            error_controller,
            loader_controller,
            children,
        } => {
            let raw_path = path.to_matchit_pattern();
            let full_pattern = if raw_path.is_empty() { "/".to_string() } else { raw_path };

            let node_id = format!("{}:P", ctx.prefix_id);

            let parallel_route = ParallelRoute {
                matched_pattern: full_pattern.clone(),
                controller: controller.clone(),
                error_controller: error_controller.clone(),
                loader_controller: loader_controller.clone(),
                layouts: ctx.layouts.clone(),
                node_id: node_id.clone(),
            };
            matcher.resolve(&full_pattern, parallel_route.clone())?;

            // ========== FIX: REMOVED the faulty optional‑catch‑all expansion ==========
            // The same fix as in normal route baking – the catch‑all pattern already covers all cases.

            for child in children {
                bake_parallel_route_recursive(child, matcher, ctx.clone())?;
            }
        }

        ParallelRouteNode::Layout {
            id,
            controller,
            error_controller,
            loader_controller,
            parallel_routes,
            children,
        } => {
            let relative_norm = normalize_to_segment(&ctx.path);
            let node_id = if ctx.path == "/" {
                format!("{}:L_{}", ctx.prefix_id, id)
            } else {
                format!("{}:L_{}_{}", ctx.prefix_id, relative_norm, id)
            };

            let mut inner_layout = Layout {
                base_path: ctx.path.clone(),
                controller,
                error_controller,
                loader_controller,
                parallel_routers: LinearMap::new(),
                node_id: node_id.clone(),
            };

            for (slot_name, nodes) in parallel_routes {
                let nested_slot_id = format!("{}:S_{}", node_id, slot_name);
                let mut slot_matcher = ParallelRouteMatcher::new();
                for node in nodes {
                    bake_parallel_route_recursive(node, &mut slot_matcher, ParallelRouteBakeContext {
                        slot_name: slot_name.clone(),
                        path: ctx.path.clone(),
                        layouts: Vec::new(),
                        prefix_id: nested_slot_id.clone(),
                    })?;
                }
                inner_layout.parallel_routers.insert(slot_name, Arc::new(slot_matcher));
            }

            let mut new_ctx = ctx.clone();
            new_ctx.layouts.push(Arc::new(inner_layout));
            new_ctx.prefix_id = node_id;

            for child in children {
                bake_parallel_route_recursive(child, matcher, new_ctx.clone())?;
            }
        }
    }
    Ok(())
}

fn normalize_to_segment(path: &str) -> String {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        "root".to_string()
    } else {
        trimmed.replace('/', ":")
    }
}

fn join_paths(parent: &str, child: &str) -> String {
    let p = parent.trim_end_matches('/');
    let c = child.trim_start_matches('/');
    if p.is_empty() {
        if c.is_empty() { "/".to_string() } else { format!("/{}", c) }
    } else if c.is_empty() { p.to_string() }
    else { format!("{}/{}", p, c) }
}