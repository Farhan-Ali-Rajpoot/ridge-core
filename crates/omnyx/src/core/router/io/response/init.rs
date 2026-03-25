use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use super::{Body, Response};





impl Response {
    pub fn html(content: impl Into<String>) -> Self {
        Self {
            body: Body::Html(content.into()),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn fragment(content: impl Into<String>) -> Self {
        Self {
            body: Body::Fragment(content.into()),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn json<T: Serialize>(data: T) -> Self {
        let val = serde_json::to_value(data).unwrap_or_else(|_| {
            serde_json::json!({ "error": "Internal serialization error" })
        });

        Self {
            body: Body::Json(val),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }

    pub fn redirect(to: impl Into<String>) -> Self {
        Self {
            body: Body::Redirect(to.into()),
            status: StatusCode::SEE_OTHER,
            headers: HeaderMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            body: Body::Empty,
            status: StatusCode::NO_CONTENT,
            headers: HeaderMap::new(),
        }
    }

    pub fn bytes(data: impl Into<Vec<u8>>, content_type: impl Into<String>) -> Self {
        Self {
            body: Body::Bytes(data.into()),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        }
    }
}

