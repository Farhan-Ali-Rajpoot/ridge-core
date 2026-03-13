use super::{
    router::{UiResolutionContext,},
    own_context::OwnContext,
    layout::LayoutProps,
};
use axum::{
    body::to_bytes,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
    Extension,
};
use std::sync::Arc;
use tracing;

pub async fn ui_middleware(
    ctx: Extension<Arc<UiResolutionContext>>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let own_ctx_value = headers
        .get("Own-Context")
        .and_then(|v| v.to_str().ok());

    let response = next.run(req).await;

    if let Some(header_value) = own_ctx_value {

        let own_ctx = OwnContext::parse(header_value);

        axum::Json(own_ctx).into_response()

    } else {
        if response.status() != StatusCode::OK {
            return response;
        }

        let (parts, body) = response.into_parts();

        let bytes = match to_bytes(body, 10_000_000).await {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("Failed to read response body: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Body read error").into_response();
            }
        };

        let mut html = match String::from_utf8(bytes.to_vec()) {
            Ok(h) => h,
            Err(e) => {
                tracing::error!("Invalid UTF-8 in response: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Encoding error").into_response();
            }
        };

        let client_stack = "";

        let client_parts: Vec<&str> = client_stack.split('/').filter(|s| !s.is_empty()).collect();
        let target_parts: Vec<&str> = ctx.stack_id.split('/').filter(|s| !s.is_empty()).collect();

        let mut common_count = 0;
        for (c, t) in client_parts.iter().zip(target_parts.iter()) {
            if c == t {
                common_count += 1;
            } else {
                break;
            }
        }

        for i in (common_count..ctx.layout_stack.len()).rev() {
            let props = LayoutProps {
                children: Some(html.into()),
                class: None,
            };
            html = (&*ctx.layout_stack[i].func)(props).await;
        }

        let mut res = (parts, Html(html)).into_response();
        if let Ok(header_value) = ctx.stack_id.parse() {
            res.headers_mut().insert("X-Response-Stack", header_value);
        }
        res
    }
}