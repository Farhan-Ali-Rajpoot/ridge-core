use std::collections::HashMap;
use std::sync::Arc;

use percent_encoding::percent_decode_str;

use axum::body::Body;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, request::Parts, Request, Uri};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use bytes::Bytes;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub id: Arc<str>,
    pub parts: Arc<Parts>,                    
    pub uri: Arc<Uri>,
    pub url: Arc<str>,
    pub path: Arc<str>,
    pub params: Arc<HashMap<String, String>>,
    pub query: Arc<HashMap<String, String>>,
    pub headers: Arc<HeaderMap>,
    pub cookies: Arc<CookieJar>,             
    pub body: Option<Arc<Bytes>>,
}

impl RequestContext {
    pub fn from_request(
        req: Request<Body>,
        params: HashMap<String, String>,
        request_id: Arc<str>,
    ) -> Self {
        let (parts, _body) = req.into_parts();  

        let uri = parts.uri.clone();
        let url = Arc::from(uri.to_string());
        let path = Arc::from(uri.path().to_string());

        let cookies = Arc::new(CookieJar::from_headers(&parts.headers.clone()));

        // Query parsing with proper percent decoding
        let mut query = HashMap::new();
        if let Some(q) = uri.query() {
            for pair in q.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    let k_decoded = percent_decode_str(k).decode_utf8_lossy().into_owned();
                    let v_decoded = percent_decode_str(v).decode_utf8_lossy().into_owned();
                    query.insert(k_decoded, v_decoded);
                } else if !pair.is_empty() {
                    // ?flag case
                    let k_decoded = percent_decode_str(pair).decode_utf8_lossy().into_owned();
                    query.insert(k_decoded, String::new());
                }
            }
        }

        Self {
            id: request_id,
            parts: Arc::new(parts.clone()),                   
            uri: Arc::new(uri),
            url,
            path,
            params: Arc::new(params),
            query: Arc::new(query),
            headers: Arc::new(parts.headers),          
            cookies,
            body: None,
        }
    }


    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(String::as_str)
    }

    pub fn get_query(&self, key: &str) -> Option<&str> {
        self.query.get(key).map(String::as_str)
    }

    pub fn get_cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    pub fn cookies(&self) -> &CookieJar {
        &self.cookies
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn get_header(&self, name: impl AsRef<str>) -> Option<&HeaderValue> {
        self.headers.get(name.as_ref())
    }

    pub fn get_header_str(&self, name: impl AsRef<str>) -> Option<&str> {
        self.get_header(name).and_then(|v| v.to_str().ok())
    }

    pub fn accepts_json(&self) -> bool {
        self.get_header_str(header::ACCEPT)
            .map_or(false, |a| a.contains("application/json"))
    }

    pub fn bearer_token(&self) -> Option<&str> {
        self.get_header_str(header::AUTHORIZATION)
            .and_then(|auth| auth.strip_prefix("Bearer ").map(str::trim))
    }

    pub fn with_body(self, body: Bytes) -> Self {
        Self {
            body: Some(Arc::new(body)),
            ..self
        }
    }

    pub fn empty() -> Self {
        let dummy_req = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap();

        Self::from_request(
            dummy_req,
            HashMap::new(),
            Arc::from("test-request-00000000"),
        )
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self::empty()
    }
}