use async_trait::async_trait;

use crate::core::router::handlers::LayoutComponent;
use crate::core::router::io::response::{Body, IntoResponse};
use crate::core::router::io::{FrameworkContext, Response};
use super::layout::LayoutProps;


pub trait FromContext: Sized {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response>;
}

// Teach FrameworkContext how to extract itself (it just clones itself!)
impl FromContext for FrameworkContext {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response> {
        Ok(ctx.clone()) 
    }
}

// Teach LayoutProps how to extract itself from the extensions map
impl FromContext for LayoutProps {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response> {
        ctx.extensions.get::<LayoutProps>()
            .cloned()
            .ok_or_else(|| Response::new(Body::Empty).with_status(500))
    }
}

// We implement LayoutComponent for a tuple of two types: (T1, T2)
#[async_trait]
impl<F, Fut, T1, T2, R> LayoutComponent<(T1, T2)> for F
where
    // F is the developer's function. It takes T1 and T2.
    F: FnOnce(T1, T2) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send,
    R: IntoResponse,
    
    // The magic rule: T1 and T2 MUST know how to extract themselves
    T1: FromContext + Send,
    T2: FromContext + Send,
{
    async fn render(self, ctx: FrameworkContext) -> Response {
        let arg1 = match T1::from_context(&ctx) {
            Ok(v) => v,
            Err(e) => return e,
        };
        
        let arg2 = match T2::from_context(&ctx) {
            Ok(v) => v,
            Err(e) => return e,
        };

        let result = self(arg1, arg2).await;
        
        // 4. Convert to your standard Response
        result.into_response()
    }
}