use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::core::router::logic::RouteMetadata;
use crate::core::router::io::{Response, FrameworkContext, IntoResponse};
use crate::error::RouteError;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutProps {
    pub children: String,                  
    pub slots: HashMap<String, String>,      
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            children: "".into(),
            slots: HashMap::new(),
        }
    }
}

impl LayoutProps {
    pub fn new(children: impl Into<String>) -> Self {
        Self {
            children: children.into(),
            ..Default::default()
        }
    }
}

#[async_trait]
pub trait FromContext: Sized {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response>;
}

impl FromContext for LayoutProps {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response> {
        Ok(ctx.layout_props.unwrap_or(Arc::new(LayoutProps::default())))
    }
}

impl FromContext for FrameworkContext {
    fn from_context(ctx: &FrameworkContext) -> Result<Self, Response> {
        Ok(ctx.clone())
    }
}

pub type LayoutFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

#[async_trait]
pub trait LayoutComponent: Send + Sync + 'static {
    type Args;
    
    async fn render(&self, ctx: FrameworkContext) -> LayoutFuture<'static, Response>;
}

#[async_trait]
impl<F, Fut, R, T1, T2> LayoutComponent for F
where
    F: Fn(T1, T2) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoResponse + Send + 'static,

    T1: FromContext + Send,
    T2: FromContext + Send,
{
    type Args = (T1, T2);

    async fn render(&self, ctx: FrameworkContext) -> LayoutFuture<'static, Response> {
        let arg1 = match T1::from_context(&ctx) {
            Ok(v) => v,
            Err(e) => return e,
        };
        
        let arg2 = match T2::from_context(&ctx) {
            Ok(v) => v,
            Err(e) => return e,
        };

        (self)(arg1, arg2)
    }
}