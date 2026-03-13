use async_trait::async_trait;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use std::sync::Arc;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Layer;

use crate::core::RidgeCore;

#[async_trait]
pub trait RidgeMiddleware: Send + Sync + 'static {
    async fn handle(
        &self,
        req: Request<axum::body::Body>,
        core: &RidgeCore,
        next: RidgeNext,
    ) -> Result<axum::response::Response, MiddlewareOutcome>;
}

pub type RidgeNext = Arc<
    dyn for<'a> Fn(
            Request<axum::body::Body>,
            &'a RidgeCore,
        ) -> Pin<Box<dyn futures::Future<Output = Result<axum::response::Response, MiddlewareOutcome>> + Send + 'a>>
        + Send
        + Sync,
>;

pub enum MiddlewareOutcome {
    Response(axum::response::Response),

    Status(StatusCode),

    ModifiedRequest(Request<axum::body::Body>),
}

impl IntoResponse for MiddlewareOutcome {
    fn into_response(self) -> axum::response::Response {
        match self {
            MiddlewareOutcome::Response(res) => res,
            MiddlewareOutcome::Status(code) => code.into_response(),
            MiddlewareOutcome::ModifiedRequest(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}