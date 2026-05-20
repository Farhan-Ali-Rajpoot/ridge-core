use std::future::Future;
use std::marker::PhantomData;

use crate::core::router::io::{request::{Request, kinds::Api}, Response};
use crate::types::BoxFuture;

pub trait ErasedApiHandler: Send + Sync + 'static {
    fn call_erased(&self, request: Request<Api>) -> BoxFuture<Response>;
}

pub trait ApiHandler<Args>: Clone + Send + Sync + 'static {
    fn call(self, request: Request<Api>) -> impl Future<Output = Response> + Send ;
}

#[derive(Debug)]
pub struct ApiHandlerWrapper<H, Args> {
    pub handler: H,
    pub _marker: PhantomData<Args>,
}

impl<H, Args> ErasedApiHandler for ApiHandlerWrapper<H, Args>
where
    H: ApiHandler<Args> + Clone + Send + Sync + 'static,
    Args: Send + Sync + 'static,
{
    fn call_erased(&self, request: Request<Api>) -> BoxFuture<Response> {
        Box::pin(self.handler.clone().call(request))
    }
}

impl_handler!(ApiHandler, call, Api; );
impl_handler!(ApiHandler, call, Api; t1);