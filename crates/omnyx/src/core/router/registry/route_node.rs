use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use axum::http::{self, Method};

use crate::core::router::logic::metadata::RouteMetadata;
use crate::core::router::logic::Middleware;
use crate::core::router::logic::DataLoader;
use crate::core::router::tree::Path;
use crate::core::router::handlers::{ApiHandler, PageComponent, SpecialComponent, LayoutComponent, ErrorComponent};


#[derive(Clone, Debug)]
pub enum SpecialNodeKind {
    Loading,
    Error,
    NotFound,
    Redirect,
    Forbidden,
}

#[derive(Clone)]
pub enum RouteNode {
    Page {
        path: Path,
        handlers: HashMap<Method, Arc<dyn PageComponent>>,
        error_handlers: HashMap<Method, Arc<dyn ErrorComponent>>,
        metadata: RouteMetadata,
        children: Vec<RouteNode>,
        loaders: Vec<Arc<dyn DataLoader>>,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
        extensions: HashMap<String, Value>,
    },
    
    Api {
        path: Path,
        handlers: HashMap<Method, Arc<dyn ApiHandler>>,
        children: Vec<RouteNode>,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
        extensions: HashMap<String, Value>,
        loaders: Vec<Arc<dyn DataLoader>>,
    },

    Layout {
        id: String,
        component: Option<Arc<dyn LayoutComponent>>,
        error_component: Option<Arc<dyn ErrorComponent>>,
        metadata: RouteMetadata,
        children: Vec<RouteNode>,
        slots: HashMap<String, Vec<RouteNode>>, 
        extensions: HashMap<String, Value>,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
        loaders: Vec<Arc<dyn DataLoader>>,
    },

    Group {
        id: String,
        children: Vec<RouteNode>,
        extensions: HashMap<String, Value>,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
        loaders: Vec<Arc<dyn DataLoader>>,
    },

    Special {
        kind: SpecialNodeKind,
        component: Option<Arc<dyn SpecialComponent>>,
        children: Vec<RouteNode>,
        middlewares: Vec<Arc<dyn Middleware + Send + Sync>>,
        loaders: Vec<Arc<dyn DataLoader>>,
        extensions: HashMap<String, Value>,
    },
}


impl std::fmt::Debug for RouteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteNode::Page { path, metadata, children, handlers, .. } => {
                f.debug_struct("RouteNode::Page")
                    .field("path", path)
                    .field("metadata", metadata)
                    .field("methods", &handlers.keys().collect::<Vec<_>>())
                    .field("children", children)
                    .finish()
            }
            RouteNode::Api { path, handlers, children, .. } => {
                f.debug_struct("RouteNode::Api")
                    .field("path", path)
                    .field("methods", &handlers.keys().collect::<Vec<_>>())
                    .field("children", children)
                    .finish()
            }
            RouteNode::Layout { id, metadata, children, slots, .. } => {
                f.debug_struct("RouteNode::Layout")
                    .field("id", id)
                    .field("metadata", metadata)
                    .field("slots", &slots.keys().collect::<Vec<_>>())
                    .field("children", children)
                    .finish()
            }
            RouteNode::Group { id, children, .. } => {
                f.debug_struct("RouteNode::Group")
                    .field("id", id)
                    .field("children", children)
                    .finish()
            }
            RouteNode::Special { kind, children, .. } => {
                f.debug_struct("RouteNode::Special")
                    .field("kind", kind)
                    .field("children", children)
                    .finish()
            }
        }
    }
}