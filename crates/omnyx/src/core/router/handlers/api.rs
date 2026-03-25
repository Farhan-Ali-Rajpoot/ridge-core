use async_trait::async_trait;
use std::future::Future;

use crate::core::router::io::{RequestContext, Response, IntoResponse};

#[async_trait]
pub trait ApiHandler: Send + Sync + 'static {
    async fn handle(&self, req: RequestContext) -> Response;
}

#[async_trait]
impl<F, Fut, R> ApiHandler for F 
where
    F: Fn(RequestContext) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoResponse + Send + 'static,
{
    async fn handle(&self, req: RequestContext) -> Response {
        (self)(req).await.into_response()
    }
}