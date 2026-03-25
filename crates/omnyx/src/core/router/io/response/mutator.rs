use axum::{
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;

use crate::core::router::io::Response;
use super::core::Body;




impl Response {
    fn status_mut(&mut self) -> &mut StatusCode {
            &mut self.status
    }

    fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn with_status(mut self, code: u16) -> Self {
        if let Ok(new_status) = StatusCode::from_u16(code) {
            *self.status_mut() = new_status;
        }
        self
    }

    pub fn with_header(
        mut self,
        key: impl Into<HeaderName>,
        value: impl Into<HeaderValue>,
    ) -> Self {
        let _ = self.headers_mut().append(key.into(), value.into());
        self
    }

    pub fn with_cookie(mut self, cookie_string: impl Into<String>) -> Self {
        if let Ok(value) = HeaderValue::try_from(cookie_string.into()) {
            self.headers_mut().append(header::SET_COOKIE, value);
        }
        self
    }

    pub fn with_cookie_typed(mut self, cookie: Cookie) -> Self {
        if let Ok(value) = HeaderValue::try_from(cookie.encoded().to_string()) {
            self.headers_mut().append(header::SET_COOKIE, value);
        }
        self
    }

    pub fn is_wrappable(&self) -> bool {
        matches!(self.body, Body::Html(_) | Body::Fragment(_))
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn extend_headers(&mut self, additional: HeaderMap) {
        for (key, value) in additional {
            if let Some(k) = key {
                self.headers.append(k, value);
            }
        }
    }
}