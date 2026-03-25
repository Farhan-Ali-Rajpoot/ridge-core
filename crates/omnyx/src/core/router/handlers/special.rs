use crate::core::router::io::{RequestContext, Response};





pub trait SpecialComponent: Send + Sync + 'static {
    fn render(&self, req: &RequestContext) -> Response;
}