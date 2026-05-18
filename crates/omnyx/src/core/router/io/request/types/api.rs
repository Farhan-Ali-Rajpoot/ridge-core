use std::sync::Arc;

use parking_lot::RwLock;

use crate::collections::LinearMap;






pub struct Api {
    // Read-only identifiers (accessible directly via Deref)
    // pub(crate) id: String,
    // pub(crate) method: http::Method,
    // pub(crate) uri: http::Uri,
    pub(crate) state: Arc<dyn std::any::Any + Send + Sync + 'static>,

    // Thread-safe mutable jars using owned Strings
    pub(crate) params: RwLock<LinearMap<String, Vec<String>>>,
    pub(crate) query: RwLock<LinearMap<String, String>>,

    // Response
    pub(crate) status: RwLock<http::StatusCode>,
    pub(crate) headers: RwLock<http::HeaderMap>,
    pub(crate) cookies: RwLock<cookie::CookieJar>,

    pub(crate) extensions: RwLock<crate::core::router::registry::Extensions>,
}