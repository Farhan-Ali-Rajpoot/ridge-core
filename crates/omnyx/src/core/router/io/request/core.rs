

#[derive(Debug, Clone)]
pub struct Request<K> {
    pub(crate) inner: K,
}

impl<K> std::ops::Deref for Request<K> {
    type Target = K;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K> Request<K> {
    pub fn new(
        inner: K,
    ) -> Request<K> {
        Self {
            inner,
        }
    }
}





// #[derive(Debug)]
// pub struct RequestInner {
//     // Thread-safe state tracking
//     pub(crate) is_dynamic: AtomicBool,
//     pub(crate) is_modified: AtomicBool,

//     // Read-only identifiers (accessible directly via Deref)
//     pub(crate) id: String,
//     pub(crate) method: Method,
//     pub(crate) uri: Uri,
//     pub(crate) layout_props: RwLock<LayoutProps>,
//     pub(crate) state: Arc<dyn std::any::Any + Send + Sync + 'static>,

//     // Thread-safe mutable jars using owned Strings
//     pub(crate) params: RwLock<LinearMap<String, Vec<String>>>,
//     pub(crate) query: RwLock<LinearMap<String, String>>,

//     // Response
//     pub(crate) status: RwLock<http::StatusCode>,
//     pub(crate) headers: RwLock<HeaderMap>,
//     pub(crate) cookies: RwLock<cookie::CookieJar>,

//     pub(crate) extensions: RwLock<crate::core::Extensions>,
//     pub(crate) metadata: RwLock<RouteMetadata>,
// }