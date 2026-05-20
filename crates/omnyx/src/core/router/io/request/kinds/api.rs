use futures::stream::{self, Stream};

use crate::core::router::io::{request::{ Request, Connection }, response::ResponseJar};

#[derive(Clone)]
pub struct Api {
    pub(crate) connection: Connection,
    pub(crate) response: ResponseJar,
}

impl std::ops::Deref for Api {
    type Target = Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl Api {
    pub(crate) fn new(
        connection: Connection
    ) -> Self {
        Self {
            connection,
            response: ResponseJar::new(),
        }
    } 
}

impl Request<Api> {
    // Get Raw body bytes
    // Developer can process raw body bytes (Lightweight Images, Svgs)
    pub async fn body_raw(&self) -> pingora::Result<bytes::Bytes> {
        unsafe {
            let session_mut = &mut *self.connection.session_ptr.as_ptr();
            const MAX_ALLOWED_BUFFER: usize = 10 * 1024 * 1024;

            let content_length = session_mut
                .req_header()
                .headers
                .get(http::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);

            // Prevent malicious clients from tricking the allocator into a panic
            let capacity = std::cmp::min(content_length, MAX_ALLOWED_BUFFER);
            let mut full_body = bytes::BytesMut::with_capacity(capacity);

            while let Some(chunk) = session_mut.read_request_body().await? {
                if full_body.len() + chunk.len() > MAX_ALLOWED_BUFFER {
                    let err = pingora::Error::explain(
                        pingora::ErrorType::HTTPStatus(413),
                        "Payload Too Large",
                    );
                    return Err(err);
                }

                full_body.extend_from_slice(&chunk);
            }

            Ok(full_body.freeze())
        }
    }

    // Get serializaed body
    // Developer can prase body into a struct
    pub async fn body<T>(&self) -> pingora::Result<T>
    where
        for<'de> T: serde::Deserialize<'de> + Send + Sync + 'static,
    {
        let bytes = self.body_raw().await?;

        let value = serde_json::from_slice::<T>(&bytes).map_err(|e| {
            pingora::Error::because(
                pingora::ErrorType::HTTPStatus(400),
                "Failed to deserialize request JSON body",
                e,
            )
        })?;

        Ok(value)
    }

    // Get stream of body (Videos, Images etc.)
    // Developer can process each chunk as they arrives
    pub fn body_stream(&self) -> impl Stream<Item = pingora::Result<bytes::Bytes>> + '_ {
        let raw_session_ptr = self.connection.session_ptr;

        stream::unfold(raw_session_ptr, |ptr| async move {
            unsafe {
                let session_mut = &mut *ptr.as_ptr();

                match session_mut.read_request_body().await {
                    Ok(Some(chunk)) => Some((Ok(chunk), ptr)),
                    Ok(None) => None,
                    Err(e) => Some((Err(e), ptr)),
                }
            }
        })
    }

    #[inline]
    pub fn response(&self) -> &ResponseJar {
        &self.inner.response
    }

}
