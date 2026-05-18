use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use parking_lot::RwLock;

use crate::collections::LinearMap;
use crate::core::router::logic::RouteMetadata;
use crate::core::router::handlers::layout::LayoutProps;





#[derive(Clone, Debug)]
pub struct Page {
    inner: Arc<PageInner>,
}

#[derive(Debug)]
pub struct PageInner {
    pub(crate) is_dynamic: AtomicBool,
    pub(crate) is_modified: AtomicBool,

    // Read-only identifiers (accessible directly via Deref)
    // pub(crate) id: String,
    // pub(crate) method: http::Method,
    // pub(crate) uri: http::Uri,
    pub(crate) layout_props: RwLock<LayoutProps>,
    pub(crate) state: Arc<dyn std::any::Any + Send + Sync + 'static>,

    // Thread-safe mutable jars using owned Strings
    pub(crate) params: RwLock<LinearMap<String, Vec<String>>>,
    pub(crate) query: RwLock<LinearMap<String, String>>,

    // Response
    pub(crate) status: RwLock<http::StatusCode>,
    pub(crate) headers: RwLock<http::HeaderMap>,
    pub(crate) cookies: RwLock<cookie::CookieJar>,

    pub(crate) extensions: RwLock<crate::core::Extensions>,
    pub(crate) metadata: RwLock<RouteMetadata>,
}

impl std::ops::Deref for Page {
    type Target = PageInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}