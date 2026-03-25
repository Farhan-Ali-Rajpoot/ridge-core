use std::borrow::Cow;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode, Uri};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::core::router::logic::RouteMetadata;

#[derive(Debug, Clone)]
pub struct ResponseContext {
    pub status: StatusCode,
    pub metadata: RouteMetadata,
    pub headers: HeaderMap,
    pub cookies: CookieJar, 
}

impl ResponseContext {
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            metadata: RouteMetadata::default(), 
            headers: HeaderMap::new(),
            cookies: CookieJar::default(),
        }
    }

    pub fn builder() -> Self {
        Self::new()
    }
}


impl ResponseContext {
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        if let (Ok(k), Ok(v)) = (key.try_into(), value.try_into()) {
            self.headers.insert(k, v);
        }
        self
    }

    pub fn append_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        if let (Ok(k), Ok(v)) = (key.try_into(), value.try_into()) {
            self.headers.append(k, v);
        }
        self
    }

    pub fn with_cookie<C>(mut self, cookie: C) -> Self
    where
        C: Into<Cookie<'static>>,
    {
        self.cookies = self.cookies.add(cookie);
        self
    }

    pub fn remove_cookie<N>(mut self, name: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        let removal = Cookie::build(name.into())
            .path("/")
            .removal()
            .build();

        self.cookies = self.cookies.add(removal);
        self
    }

    pub fn redirect_to(self, location: &str, permanent: bool) -> Self {
        let status = if permanent {
            StatusCode::PERMANENT_REDIRECT
        } else {
            StatusCode::FOUND
        };

        self.with_status(status)
            .with_header(header::LOCATION, location)
    }

    pub fn as_json(mut self) -> Self {
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        self
    }

    pub fn as_html(mut self) -> Self {
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        self
    }

    pub fn as_text(mut self) -> Self {
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        self
    }
}


impl ResponseContext {
    pub fn apply_to_axum<B>(&self, mut response: axum::response::Response<B>) -> axum::response::Response<B> {
        *response.status_mut() = self.status;

        response.headers_mut().extend(self.headers.clone());

        for cookie in self.cookies.iter() {
            if let Ok(value) = HeaderValue::from_str(&cookie.encoded().to_string()) {
                response.headers_mut().append(header::SET_COOKIE, value);
            }
        }

        response
    }

    pub fn into_axum_response<B>(&self, body: B) -> axum::response::Response
    where
        B: Into<axum::body::Body>,
    {
        let mut res = axum::response::Response::new(body.into());
        self.apply_to_axum(res)
    }
}

impl Default for ResponseContext {
    fn default() -> Self {
        Self::new()
    }
}