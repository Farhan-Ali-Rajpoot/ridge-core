use std::future::Future;
use axum::response::{IntoResponse, Response};
use async_trait::async_trait;

use crate::error::RouteError;
use crate::render::layout::{LayoutFn, LayoutProps};
use crate::core::router::RequestContext;

pub type RouteResponse = Result<Response, RouteError>;
pub type PageResponse = String;

#[async_trait]
pub trait PageHandler: Send + Sync + 'static {
    async fn render(&self, ctx: &RequestContext) -> Result<PageResponse, RouteError>;
}

#[async_trait]
pub trait ApiHandler: Send + Sync + 'static {
    async fn handle(&self, ctx: &RequestContext) -> RouteResponse;
}

// RouteAction (maybe for forms, etc.)
#[async_trait]
pub trait RouteAction: Send + Sync + 'static {
    async fn execute(&self, ctx: &RequestContext) -> RouteResponse;
}

// SpecialComponent (error pages, loading, etc.)
pub trait SpecialComponent: Send + Sync + 'static {
    fn render(&self, ctx: &RequestContext) -> Response;
}

// ========== Page Handler Factory ==========
pub fn make_page_handler<F, Fut>(f: F) -> impl PageHandler
where
    F: Fn(&RequestContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<String, RouteError>> + Send,
{
    #[derive(Clone)]
    struct PageHandlerClosure<F>(F);

    #[async_trait]
    impl<F, Fut> PageHandler for PageHandlerClosure<F>
    where
        F: Fn(&RequestContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, RouteError>> + Send,
    {
        async fn render(&self, ctx: &RequestContext) -> Result<String, RouteError> {
            (self.0)(ctx).await
        }
    }

    PageHandlerClosure(f)
}

#[macro_export]
macro_rules! page {
    ($closure:expr) => {
        $crate::router::handler::make_page_handler($closure)
    };
}

// ========== API Handler Factory ==========
pub fn make_api_handler<F, Fut, Res>(f: F) -> impl ApiHandler
where
    F: Fn(&RequestContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Res, RouteError>> + Send,
    Res: IntoResponse,
{
    #[derive(Clone)]
    struct ApiHandlerClosure<F>(F);

    #[async_trait]
    impl<F, Fut, Res> ApiHandler for ApiHandlerClosure<F>
    where
        F: Fn(&RequestContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Res, RouteError>> + Send,
        Res: IntoResponse,
    {
        async fn handle(&self, ctx: &RequestContext) -> RouteResponse {
            (self.0)(ctx).await.map(|res| res.into_response())
        }
    }

    ApiHandlerClosure(f)
}

#[macro_export]
macro_rules! api {
    ($closure:expr) => {
        $crate::router::handler::make_api_handler($closure)
    };
}

// ========== Layout Factories (unchanged, but fixed return types) ==========
pub fn make_full_layout<F, Fut>(f: F) -> impl LayoutFn + Clone + Send + Sync + 'static
where
    F: Fn(&RequestContext, LayoutProps) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send,
{
    #[derive(Clone)]
    struct FullLayout<F>(F);

    #[async_trait]
    impl<F, Fut> LayoutFn for FullLayout<F>
    where
        F: Fn(&RequestContext, LayoutProps) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = String> + Send,
    {
        async fn render(&self, props: LayoutProps, ctx: &RequestContext) -> String {
            (self.0)(ctx, props).await
        }
    }

    FullLayout(f)
}

#[macro_export]
macro_rules! layout_full {
    ($closure:expr) => {
        $crate::router::handler::make_full_layout($closure)
    };
}

pub fn make_simple_layout<F>(f: F) -> impl LayoutFn + Clone + Send + Sync + 'static
where
    F: Fn(LayoutProps) -> String + Send + Sync + 'static,
{
    #[derive(Clone)]
    struct SimpleLayout<F>(F);

    #[async_trait]
    impl<F> LayoutFn for SimpleLayout<F>
    where
        F: Fn(LayoutProps) -> String + Send + Sync + 'static,
    {
        async fn render(&self, props: LayoutProps, _ctx: &RequestContext) -> String {
            (self.0)(props)
        }
    }

    SimpleLayout(f)
}

#[macro_export]
macro_rules! layout {
    ($closure:expr) => {
        $crate::router::handler::make_simple_layout($closure)
    };
}