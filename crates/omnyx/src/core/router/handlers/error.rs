use std::future::Future;
use async_trait::async_trait;
use crate::core::router::io::{RequestContext, Response, IntoResponse};

#[async_trait]
pub trait ErrorComponent: Send + Sync + 'static {
    async fn render(&self, ctx: RequestContext) -> Response;
}

#[async_trait]
impl<F, Fut, R> ErrorComponent for F 
where
    F: Fn(RequestContext) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoResponse + Send + 'static, 
{
    async fn render(&self, ctx: RequestContext) -> Response {
        (self)(ctx).await.into_response()
    }
}