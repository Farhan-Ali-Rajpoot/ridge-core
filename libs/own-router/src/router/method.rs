use super::{
    ui_middleware,
    metadata::{RouteMetadata},
    layout::LayoutProps,
    router::{UiResolutionContext, LayoutNode, RouteNode, RouterBuilder, AsyncLayoutFn,},
};
use axum::{
    extract::Request,
    middleware::Next,
    response::{Html, IntoResponse, Response},
    routing::MethodRouter,
    Extension, Router,
};
use std::{
    future::Future,
    sync::Arc,
};



impl<S> RouterBuilder<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            layouts: Vec::new(),
            routes: Vec::new(),
            stack_id: Arc::from(""),
        }
    }

    pub fn layout<F, Fut>(
        mut self,
        id: &'static str,
        layout_fn: F,
        metadata: Option<RouteMetadata>,
    ) -> Self
    where
        F: Fn(LayoutProps) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        let func: Arc<AsyncLayoutFn> = Arc::new(move |props| {
            let f = layout_fn.clone();
            Box::pin(async move { f(props).await })
        });
        self.layouts.push(LayoutNode {
            func,
            metadata: Arc::new(metadata.unwrap_or(RouteMetadata::default())),
        });

        let new_id = if self.stack_id.is_empty() {
            id.to_string()
        } else {
            format!("{}/{}", self.stack_id, id)
        };
        self.stack_id = Arc::from(new_id);
        self
    }

    pub fn route(
        mut self,
        path: &'static str,
        method_router: MethodRouter<S>,
        metadata: Option<RouteMetadata>,
    ) -> Self {
        self.routes.push(RouteNode {
            path: Arc::from(path),
            method_router,
            metadata: metadata.map(Arc::new),
        });
        self
    }

    pub fn nest(mut self, path: &'static str, mut child: Self) -> Self {

        let parent_layouts = self.layouts.clone();

        child.layouts.splice(0..0, parent_layouts);

        if !self.stack_id.is_empty() {
            let new_id = format!("{}/{}", self.stack_id, child.stack_id);
            child.stack_id = Arc::from(new_id);
        } else {
            child.stack_id = self.stack_id.clone();
        }

        let child_router = child.build();
        self.router = self.router.nest(path, child_router);
        self
    }

    pub fn compose(mut self, mut child: Self) -> Self {

        for route in &child.routes {
            if self.routes.iter().any(|r| r.path == route.path) {
                panic!("Route conflict in compose(): '{}' already defined", route.path);
            }
        }

        let parent_layouts = self.layouts.clone();

        child.layouts.splice(0..0, parent_layouts);

        if !self.stack_id.is_empty() {
            let new_id = format!("{}/{}", self.stack_id, child.stack_id);
            child.stack_id = Arc::from(new_id);
        } else {
            child.stack_id = self.stack_id.clone();
        }

        let child_router = child.build();
        self.router = self.router.merge(child_router);

        self
    }

    pub fn merge(mut self, mut child: Self) -> Self {
        for route in &child.routes {
            if self.routes.iter().any(|r| r.path == route.path) {
                panic!("Route conflict in merge(): '{}' already defined", route.path);
            }
        }
        self.routes.append(&mut child.routes);
        self.router = self.router.merge(child.build());
        self
    }

    
}