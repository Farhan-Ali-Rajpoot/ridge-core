use std::borrow::Cow;
use serde_json::Value;
use axum::http::StatusCode;
use axum::response::IntoResponse as AxumIntoResponse;
use super::{Response, Body};

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Self {
        self
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new(Body::Html(self))
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::new(Body::Html(self.into()))
    }
}

impl IntoResponse for Cow<'static, str> {
    fn into_response(self) -> Response {
        Response::new(Body::Html(self.into_owned()))
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::Bytes(self));
        res.headers.insert(
            axum::http::header::CONTENT_TYPE, 
            axum::http::HeaderValue::from_static("application/octet-stream")
        );
        res
    }
}

impl IntoResponse for serde_json::Value {
    fn into_response(self) -> Response {
        Response::new(Body::Json(self))
    }
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        Response::new(Body::Empty)
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse, 
{
    fn into_response(self) -> Response {
        match self {
            Ok(value) => value.into_response(),
            Err(err) => {
                let mut res = err.into_response();
                if res.status == StatusCode::OK {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR;
                }
                res
            }
        }
    }
}

impl<T> IntoResponse for Option<T> 
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Some(value) => value.into_response(),
            None => Response::new(Body::Empty),
        }
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::Empty);
        res.status = self;
        res
    }
}

// The conversion to Axum (This is where you had the E0034 error)
impl AxumIntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let mut res = match self.body {
            Body::Html(c) | Body::Fragment(c) => axum::response::Html(c).into_response(),
            Body::Json(v) => axum::response::Json(v).into_response(),
            Body::Bytes(b) => axum::response::IntoResponse::into_response(b),
            Body::Redirect(url) => axum::response::Redirect::to(&url).into_response(),
            // FIX E0034: Explicitly call Axum's trait for the unit type
            Body::Empty => axum::response::IntoResponse::into_response(()),
        };

        *res.status_mut() = self.status;
        res.headers_mut().extend(self.headers);
        res
    }
}