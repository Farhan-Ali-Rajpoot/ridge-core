use std::marker::PhantomData;

use crate::collections::LinearMap;
use crate::core::router::io::{request::{Request, kinds::Page}, Response};
use crate::types::BoxFuture;

#[derive(Debug, Clone, Default)]
pub struct RenderedParallelRoute {
    pub html: String,
    pub params: LinearMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutProps {
    pub children: String,                  
    pub parallel_routes: LinearMap<String, RenderedParallelRoute>,     
    pub node_id: String,
}

impl LayoutProps {
    pub fn new(children: impl Into<String>) -> Self {
        Self {
            children: children.into(),
            ..Default::default()
        }
    }
}
pub trait ErasedLayoutComponent: Send + Sync + 'static {
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response>;
}

pub trait LayoutComponent<Args>: Clone + Send + Sync + 'static {
    fn call(self, request: Request<Page>) -> BoxFuture<Response>;
}

pub struct LayoutComponentWrapper<H, Args> {
    pub handler: H,
    pub _marker: PhantomData<Args>,
}

impl<H, Args> ErasedLayoutComponent for LayoutComponentWrapper<H, Args>
where
    H: LayoutComponent<Args> + Clone + Send + Sync + 'static,
    Args: Send + Sync + Clone + 'static,
{
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response> {
        self.handler.clone().call(request)
    }
}

impl_handler!(LayoutComponent, call, Page; );
impl_handler!(LayoutComponent, call, Page; t1);
impl_handler!(LayoutComponent, call, Page; t1, t2);