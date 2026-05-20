use crate::core::router::io::{Request};
use crate::core::router::handlers::{LayoutProps};
use crate::core::router::io::request::kinds::Page;


pub trait FromContext<K>: Sized {
    fn from_request(request: &Request<K>) -> impl std::future::Future<Output = Self> + Send;
}


impl<K: Clone + Send + Sync + 'static> FromContext<K> for Request<K> {
    fn from_request(request: &Request<K>) -> impl Future<Output = Self> + Send {
        // Create an owned copy outside or inside the async block
        let owned_request = request.clone();
        async move {
            owned_request
        }
    }
}

impl FromContext<Page> for LayoutProps {
    fn from_request(request: &Request<Page>) -> impl std::future::Future<Output = Self> + Send {
        async move {
            // Acquires the single lock we built inside Request<Page>
            request.layout_props().clone()
        }
    }
}
