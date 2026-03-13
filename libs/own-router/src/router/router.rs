use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
};

use axum::{
    extract::Request,
    middleware::Next,
    response::{Html, IntoResponse, Response},
    routing::MethodRouter,
    Extension, Router,
};
use serde_json;

use super::{
    ui_middleware,
    metadata::{RouteMetadata},
    layout::LayoutProps,
};

pub type PageRouter<S = ()> = RouterBuilder<S>;


pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type AsyncLayoutFn = dyn Fn(LayoutProps) -> BoxFuture<'static, String> + Send + Sync;

pub struct ResolvedMetadata {
    pub html: String,
    pub json: serde_json::Value,
}


#[derive(Clone)]
pub struct LayoutNode {
    pub func: Arc<AsyncLayoutFn>,
    pub metadata: Arc<RouteMetadata>,
}

#[derive(Clone)]
pub struct RouteNode<S> {
    pub path: Arc<str>,
    pub method_router: MethodRouter<S>,
    pub metadata: Option<Arc<RouteMetadata>>,
}

pub struct RouterBuilder<S = ()> {
    pub router: Router<S>,
    pub layouts: Vec<LayoutNode>,
    pub routes: Vec<RouteNode<S>>,         
    pub stack_id: Arc<str>,                 
}

#[derive(Clone)]
pub struct UiResolutionContext {
    pub stack_id: Arc<str>,
    pub layout_stack: Arc<Vec<LayoutNode>>,
    pub metadata: Arc<ResolvedMetadata>,
}






impl<S> RouterBuilder<S>
where
    S: Clone + Send + Sync + 'static
{
    pub fn build(mut self) -> Router<S> {
        let routes = std::mem::take(&mut self.routes);

        let base_metadata = self.layouts.iter().fold(
            RouteMetadata::default(),
            |acc, layout| acc.inherit_from(&layout.metadata),
        );
        
        let base_metadata = Arc::new(base_metadata);

        let shared_layouts = Arc::new(self.layouts);

        for route_node in routes {
            let final_metadata = match &route_node.metadata {
                Some(route_meta) => Arc::new(route_meta.inherit_from(&base_metadata)),
                None => base_metadata.clone(),
            };

            let html = final_metadata.render_html();
            let json = final_metadata.to_flat_json();

            let resolved = Arc::new(ResolvedMetadata { html, json });

            let ctx = Arc::new(UiResolutionContext {
                stack_id: self.stack_id.clone(),
                layout_stack: shared_layouts.clone(),
                metadata: resolved,
            });

            let method_router = route_node.method_router
                .layer::<_, std::convert::Infallible>(axum::middleware::from_fn(ui_middleware))
                .layer(Extension(ctx));

            self.router = self.router.route(&route_node.path, method_router);
        }

        self.router
    }
}