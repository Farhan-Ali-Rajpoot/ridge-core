use pingora::proxy::Session;

use crate::core::router::io::request::{Request, kinds::Api};
use crate::core::{PingoraAdapter, Response};

impl<T> PingoraAdapter<T>
where
    T: Send + Sync + 'static
{
    pub async fn finalize_response(
        &self,
        session: &mut Session,
        req: &Request<Api>,
        response: Option<Response>,
    ) -> pingora::Result<bool> {
        
        let mut header = req.inner.response.to_response_header();

        let body_bytes = if let Some(res) = response {
            let (bytes, content_type) = res.body.into_bytes_and_content_type();
            
            if let Ok(ct_val) = http::HeaderValue::from_str(&content_type) {
                header.insert_header(http::header::CONTENT_TYPE, ct_val).unwrap();
            }
            
            if let Ok(cl_val) = http::HeaderValue::from_str(&bytes.len().to_string()) {
                header.insert_header(http::header::CONTENT_LENGTH, cl_val).unwrap();
            }
            Some(bytes)
        } else {
            None
        };

        session
            .write_response_header(header, body_bytes.is_none())
            .await?;

        if let Some(bytes) = body_bytes {
            session
                .write_response_body(Some(bytes), true)
                .await?;
        }

        Ok(true)
    }
}
