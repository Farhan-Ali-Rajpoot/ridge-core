use std::marker::PhantomData;

use crate::core::router::io::{request::{Request, kinds::Page}, Response};
use crate::types::BoxFuture;


pub trait ErasedNotFoundComponent: Send + Sync + 'static {
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response>;
}

pub trait NotFoundComponent<Args>: Clone + Send + Sync + 'static {
    fn call(self, request: Request<Page>) -> BoxFuture<Response>;
}


pub struct NotFoundComponentWrapper<H, Args> {
    pub handler: H,
    pub _marker: PhantomData<Args>,
}

impl<H, Args> ErasedNotFoundComponent for NotFoundComponentWrapper<H, Args>
where
    H: NotFoundComponent<Args> + Clone + Send + Sync + 'static,
    Args: 'static + Send + Sync + Clone,
{
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response> {
        self.handler.clone().call(request)
    }
}

impl_handler!(NotFoundComponent, call, Page; );
impl_handler!(NotFoundComponent, call, Page; t1);