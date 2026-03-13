use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use async_trait::async_trait;
use axum::{
    body::Body,
    http::{self, Method, Request},
};

use crate::core::router::metadata::RouteMetadata;
use crate::render::layout::LayoutFn;
use crate::middleware::RidgeMiddleware;
use crate::loader::DataLoader;
use crate::render::mode::RenderMode;
use crate::error::RouteError;
use crate::core::router::{RouteAction, RouteHandler, ApiHandler, SpecialComponent};

use crate::core::router::RequestContext;
use crate::core::router::PathSegment;


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
        segment: PathSegment,
        handlers: HashMap<Method, Arc<dyn RouteHandler>>,
        metadata: RouteMetadata,
        children: Vec<RouteNode>,
        loaders: Vec<Arc<dyn DataLoader>>,
        middlewares: Vec<Arc<dyn RidgeMiddleware + Send + Sync>>,
        actions: Vec<Arc<dyn RouteAction>>,
        render_mode: Option<RenderMode>,
        extensions: HashMap<String, Value>,
    },
    
    Api {
        segment: PathSegment,
        handlers: HashMap<Method, Arc<dyn ApiHandler>>,
        children: Vec<RouteNode>,
        middlewares: Vec<Arc<dyn RidgeMiddleware + Send + Sync>>,
        extensions: HashMap<String, Value>,
    },

    Layout {
        id: String,
        component: Option<Arc<dyn LayoutFn>>,
        metadata: RouteMetadata,
        children: Vec<RouteNode>,
        parallel_slots: HashMap<String, Vec<RouteNode>>, 
        extensions: HashMap<String, Value>,
    },

    Group {
        id: String,
        children: Vec<RouteNode>,
        extensions: HashMap<String, Value>,
    },

    MiddlewareBoundary {
        middlewares: Vec<Arc<dyn RidgeMiddleware + Send + Sync>>,
        children: Vec<RouteNode>,
    },

    Special {
        kind: SpecialNodeKind,
        component: Option<Arc<dyn SpecialComponent>>,
        children: Vec<RouteNode>,
    },

    Extension {
        node_type: String, // "trpc-procedure", "liveview-channel", "graphql-endpoint"...
        data: Value,
        children: Vec<RouteNode>,
    },
}
