use std::marker::PhantomData;

use crate::core::router::io::{request::{Request, kinds::Page}, Response};
use crate::types::BoxFuture;

pub trait ErasedSpecialComponent: Send + Sync + 'static {
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response>;
}


pub trait SpecialComponent<Args>: Clone + Send + Sync + 'static {
    fn call(self, request: Request<Page>) -> BoxFuture<Response>;
}

pub struct SpecialHandlerWrapper<H, Args> {
    pub handler: H,
    pub _marker: PhantomData<Args>,
}


impl<H, Args> ErasedSpecialComponent for SpecialHandlerWrapper<H, Args>
where
    H: SpecialComponent<Args> + Clone + Send + Sync + 'static,
    Args: Send + Sync + 'static,
{
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response> {
        Box::pin(self.handler.clone().call(request))
    }
}

impl_handler!(SpecialComponent, call, Page; );
impl_handler!(SpecialComponent, call, Page; t1);