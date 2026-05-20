use bytes::Bytes;
use http::HeaderValue;
use mime_guess::from_path;
use pingora::{http::ResponseHeader, proxy::Session};

use crate::core::PingoraAdapter;

impl<T> PingoraAdapter<T> where T: Send + Sync + 'static {

    /// Handles requests to the embedded public directory (via include_dir).
    pub async fn serve_public_directory(&self, session: &mut Session, path: &str) -> pingora::Result<bool> {
        if !path.starts_with("/public") {
            return Ok(false);
        }

        let relative = path.strip_prefix("/public").unwrap_or(path).trim_start_matches('/');
        if let Some((bytes, mime)) = self.serve_embedded_file(relative) {
            let mut header = ResponseHeader::build(200, None).unwrap();
            header.insert_header("Content-Type", mime).unwrap();
            // Set Content-Length to prevent layout shift / FOUC
            header.insert_header("Content-Length", bytes.len().to_string()).unwrap();
            if let Ok(cache_val) = HeaderValue::from_str("public, max-age=86400") {
                header.insert_header("Cache-Control", cache_val).unwrap();
            }
            session.write_response_header(Box::new(header), false).await?;
            session.write_response_body(Some(bytes), true).await?;
            return Ok(true);
        }

        // eprintln!("[handle_public_response] File not found: {}", path);
        self.renderer.handle_not_found_response(session).await
    }

    /// Retrieves a file from the embedded public directory (if configured).
    fn serve_embedded_file(&self, rel_path: &str) -> Option<(Bytes, String)> {
        let path = if rel_path.is_empty() || rel_path == "/" {
            "index.html"
        } else {
            rel_path
        };
        // Basic security
        if path.contains("..") || path.starts_with('/') {
            return None;
        }
        let dir = self.state.config.embedded_public_dir?;
        let file = dir.get_file(path)?;
        let bytes = Bytes::from(file.contents());
        let mime = from_path(path).first_or_octet_stream().to_string();
        Some((bytes, mime))
    }
}