use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};



#[derive(Debug, Default, Clone)]
pub struct ResponseJar {
    pub status: Option<StatusCode>,
    pub headers: Option<HeaderMap>,
}

impl ResponseJar {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResponseJar {
    #[inline]
    pub(crate) fn to_response_header(&self) -> Box<pingora::http::ResponseHeader> {
        // 1. Resolve Status Code (Fall back to 200 OK if left as None)
        let status_code = self.status.unwrap_or(http::StatusCode::OK);
        
        // 2. Instantiate Pingora's native structural layout
        let mut pingora_header = pingora::http::ResponseHeader::build(status_code.as_u16(), None).unwrap();

        // 3. Populate only if custom operations were explicitly executed
        if let Some(ref custom_headers) = self.headers {
            for (name, value) in custom_headers.iter() {
                pingora_header.insert_header(name, value).unwrap();
            }
        }

        Box::new(pingora_header)
    }

    // --- Public Api ---
    #[inline]
    pub fn set_status(&mut self, code: StatusCode) {
        self.status = Some(code);
    }

    /// Read the current status code if it has been explicitly configured
    #[inline]
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    /// Safely fetches or initializes the underlying HeaderMap allocation inline
    #[inline(always)]
    fn ensure_headers(&mut self) -> &mut HeaderMap {
        self.headers.get_or_insert_with(HeaderMap::new)
    }

    /// Insert or overwrite an explicit header value inside the lazy allocation
    #[inline]
    pub fn set_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.ensure_headers().insert(name, value);
    }

    /// Append an extra header entry under an existing key without wiping previous ones
    #[inline]
    pub fn append_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.ensure_headers().append(name, value);
    }

    /// Retrieve a reference to a header value string slice if it exists in the lazy map
    #[inline]
    pub fn header(&self, name: &HeaderName) -> Option<&str> {
        self.headers.as_ref()
            .and_then(|h| h.get(name))
            .and_then(|v| v.to_str().ok())
    }

    /// Fast structural scan to verify if a specific header has been added by prior layers
    #[inline]
    pub fn has_header(&self, name: &HeaderName) -> bool {
        self.headers.as_ref()
            .map(|h| h.contains_key(name))
            .unwrap_or(false)
    }

    /// Append a standard Set-Cookie string slice into the existing HTTP header structure
    #[inline]
    pub fn set_cookie(&mut self, name: &str, value: &str) {
        let cookie_str = format!("{}={}; Path=/; HttpOnly; SameSite=Lax", name, value);
        if let Ok(hv) = HeaderValue::from_str(&cookie_str) {
            self.ensure_headers().append(http::header::SET_COOKIE, hv);
        }
    }

    /// Scans accumulated outbound cookies to find a matching cookie name value
    #[inline]
    pub fn cookie(&self, name: &str) -> Option<&str> {
        let headers = self.headers.as_ref()?;
        
        // Zero allocations: iterates purely over the raw Set-Cookie slices
        headers.get_all(http::header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(|pair| pair.trim())
            .filter_map(|pair| pair.split_once('='))
            .find(|(k, _)| *k == name)
            .map(|(_, v_and_attrs)| {
                // Strip trailing cookie attributes (Path, HttpOnly, etc.) to yield only the token value
                v_and_attrs.split(';').next().unwrap_or(v_and_attrs).trim()
            })
    }

    /// Check if a specific cookie key has already been added to the outbound jar
    #[inline]
    pub fn has_cookie(&self, name: &str) -> bool {
        self.cookie(name).is_some()
    }
}