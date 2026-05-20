use pingora::proxy::Session;
use bytes::Bytes;

use crate::core::router::io::request::{Request, Page};
use crate::core::{Body, DeferredTask, Renderer, Response};

impl Renderer {
    pub(crate) async fn finalize_streaming_response(
        &self,
        session: &mut Session,
        req: &Request<Page>,
        response: Option<Response>,
        tasks: Option<Vec<DeferredTask>>,
    ) -> pingora::Result<bool> {
        
        let mut header = req.inner.response.read().to_response_header();

        let body_bytes = if let Some(res) = response {
            let (bytes, content_type) = res.body.into_bytes_and_content_type();
            
            if let Ok(ct_val) = http::HeaderValue::from_str(&content_type) {
                header.insert_header(http::header::CONTENT_TYPE, ct_val).unwrap();
            }
            
            header.remove_header(http::header::CONTENT_LENGTH.as_str());
            Some(bytes)
        } else {
            None
        };

        session
            .write_response_header(header, false)
            .await?;

        session
            .write_response_body(body_bytes, matches!(tasks, None))
            .await?;

        // 3. Streaming tasks handling
        if let Some(mut tasks) = tasks {
            for task in tasks.drain(..).rev() {
                let res = task.task.await;
                let html = if matches!(res.body, Body::Err(_)) {
                    // Fall back gracefully to custom error controllers
                    if let Some(err_ctr) = &task.error_controller {
                        let err_res = err_ctr.call_erased(req.clone()).await;
                        if !matches!(err_res.body, Body::Err(_)) {
                            err_res.body.to_string()
                        } else {
                            format!("Error occurred in Error Handler: {}", err_res.body.to_string())
                        }
                    } else {
                        format!("Error occurred: {} (no error handler is defined)", res.body.to_string())
                    }
                } else {
                    // Render standard dynamic successful stream task blocks
                    res.body.to_string()
                };
                
                let chunk = format!(
                    r#"<template id="tpl-{}">{}</template><script>(function(id){{var p=document.getElementById(id),t=document.getElementById("tpl-"+id);p&&t&&p.replaceWith(t.content.cloneNode(!0));console.log("Replacing", id);}})("{}");</script>"#,
                    task.id, html, task.id
                );
                
                session
                    .write_response_body(Some(Bytes::from(chunk)), false)
                    .await?;
            }
        }

        // Closing connection
        session.write_response_body(None, true).await?;
        Ok(true)
    }
}
