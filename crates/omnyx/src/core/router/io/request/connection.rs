use http::{HeaderMap, Method, Uri};
use pingora::proxy::Session;
use std::borrow::Cow;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::collections::LinearMap;

#[derive(Debug, Clone)]
pub struct Connection {
    pub state: Arc<dyn std::any::Any + Send + Sync + 'static>,
    pub(crate) session_ptr: NonNull<Session>,
    pub(crate) extensions: crate::core::router::registry::Extensions,
    pub(crate) params: LinearMap<String, Vec<String>>,
    _marker: std::marker::PhantomData<&'static Session>,
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

impl Connection {
    pub(crate) fn new(
        state: Arc<dyn std::any::Any + Send + Sync + 'static>,
        session_ptr: &mut Session,
        params: LinearMap<String, Vec<String>>,
    ) -> Self {
        Self {
            state,
            session_ptr: NonNull::new(session_ptr).unwrap(),
            extensions: crate::core::router::registry::Extensions::new(),
            params,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    fn session(&self) -> &Session {
        unsafe { self.session_ptr.as_ref() }
    }

    /// HTTP method from the request line.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.session().req_header().method
    }

    /// Request URI.
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.session().req_header().uri
    }

    #[inline]
    pub fn param(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get(key)
    }

    #[inline]
    pub fn param_first(&self, key: &str) -> Option<&str> {
        self.params
            .get(key)
            .and_then(|values| values.first())
            .map(|s| s.as_str())
    }

    // Get Params
    #[inline]
    pub fn params_raw(&self) -> &LinearMap<String, Vec<String>> {
        &self.params
    }

    // Check Params
    #[inline]
    pub fn has_param(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    #[inline]
    pub fn queries(&self) -> impl Iterator<Item = (Cow<'_, str>, Cow<'_, str>)> {
        let raw_query = self.uri().query().unwrap_or("");
        form_urlencoded::parse(raw_query.as_bytes())
    }

    /// Get the first value of a query parameter.
    /// Returns `None` if the key does not exist or the query string is empty.
    #[inline]
    pub fn query(&self, key: &str) -> Option<Cow<'_, str>> {
        let raw_query = self.uri().query()?;
        form_urlencoded::parse(raw_query.as_bytes())
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Check existence of a query key without copying any data.
    #[inline]
    pub fn has_query(&self, key: &str) -> bool {
        self.uri()
            .query()
            .map(|raw| form_urlencoded::parse(raw.as_bytes()).any(|(k, _)| k == key))
            .unwrap_or(false)
    }

    /// Get the first value of a header as a `&str`.
    /// Returns `None` if the header is missing or contains invalid UTF‑8.
    /// (Invalid UTF‑8 is silently ignored – log if you need to detect attacks.)
    #[inline]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers_raw()
            .get(name)
            .and_then(|v| v.to_str().ok())
    }

    /// Check if a header exists (case‑insensitive).
    #[inline]
    pub fn has_header(&self, name: &str) -> bool {
        self.headers_raw().contains_key(name)
    }

    /// Raw `HeaderMap` reference for advanced use.
    #[inline]
    pub fn headers_raw(&self) -> &HeaderMap {
        &self.session().req_header().headers
    }

    /// Iterator over all values of a multi‑value header.
    /// Skips values that are not valid UTF‑8.
    #[inline]
    pub fn header_all(&self, name: &str) -> impl Iterator<Item = &str> {
        self.headers_raw()
            .get_all(name)
            .iter()
            .filter_map(|v| v.to_str().ok())
    }

    /// Get the raw `Cookie` header value (if any).
    #[inline]
    pub fn cookies_raw(&self) -> Option<&str> {
        self.header("Cookie")
    }

    /// Check if a specific cookie exists (case‑insensitive name).
    #[inline]
    pub fn has_cookie(&self, name: &str) -> bool {
        self.cookie(name).is_some()
    }

    #[inline]
    pub fn cookie(&self, name: &str) -> Option<&str> {
        let cookie_header = self.cookies_raw()?;
        if cookie_header.is_empty() {
            return None;
        }

        for pair in cookie_header.split(';') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            if let Some((k, v)) = pair.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                // Case‑insensitive comparison per RFC 6265
                if k.eq_ignore_ascii_case(name) {
                    return Some(v);
                }
            }
        }
        None
    }

    #[inline]
    pub fn cookie_all(&self) -> impl Iterator<Item = (&str, &str)> {
        let cookie_header = self.cookies_raw().unwrap_or("");
        cookie_header
            .split(';')
            .filter_map(|pair| {
                let pair = pair.trim();
                if pair.is_empty() {
                    return None;
                }
                pair.split_once('=').map(|(k, v)| (k.trim(), v.trim()))
            })
    }
}
