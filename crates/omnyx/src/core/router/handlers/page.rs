use std::future::Future;
use async_trait::async_trait;

use crate::core::router::io::{RequestContext, Response, IntoResponse};
use crate::error::RouteError;





#[async_trait]
pub trait PageComponent: Send + Sync + 'static {
    async fn render(&self, req: RequestContext) -> Response;
}

#[async_trait]
impl<F, Fut, R> PageComponent for F 
where
    F: Fn(RequestContext) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoResponse + Send + 'static,
{
    async fn render(&self, req: RequestContext) -> Response {
        (self)(req).await.into_response()
    }
}


