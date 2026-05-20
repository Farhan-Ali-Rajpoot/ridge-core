use std::marker::PhantomData;

use crate::core::router::io::{request::{Request, kinds::Page}, Response};
use crate::types::BoxFuture;


pub trait ErasedPageComponent: Send + Sync + 'static {
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response>;
}

pub trait PageComponent<Args>: Clone + Send + Sync + 'static {
    fn call(self, request: Request<Page>) -> BoxFuture<Response>;
}


pub struct PageComponentWrapper<H, Args> {
    pub handler: H,
    pub _marker: PhantomData<Args>,
}

impl<H, Args> ErasedPageComponent for PageComponentWrapper<H, Args>
where
    H: PageComponent<Args> + Clone + Send + Sync + 'static,
    Args: 'static + Send + Sync + Clone,
{
    fn call_erased(&self, request: Request<Page>) -> BoxFuture<Response> {
        self.handler.clone().call(request)
    }
}

impl_handler!(PageComponent, call, Page; );
impl_handler!(PageComponent, call, Page; t1);