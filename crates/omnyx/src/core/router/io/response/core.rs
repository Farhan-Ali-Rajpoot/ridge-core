use axum::http::{HeaderMap, StatusCode};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Body {
    Html(String),
    Fragment(String),
    Json(Value),
    Bytes(Vec<u8>),
    Redirect(String),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Body,
}

impl Response {
    pub fn new(body: Body) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body,
        }
    }
}