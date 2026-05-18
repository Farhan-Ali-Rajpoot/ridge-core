use pingora::proxy::Session;

use crate::core::pingora::PingoraAdapter;
use crate::core::router::io::{Request,};
use crate::core::{RouteKind, RouteMetadata};



impl<T> PingoraAdapter<T>
where 
    T: Send + Sync + 'static
{
    pub(crate) async fn handle_route(&self, session: &mut Session) -> pingora::Result<bool> {
        let raw_path = session.req_header().uri.path();
        let path = if raw_path.len() > 1 {
            raw_path.trim_end_matches("/").to_string()
        } else {
            raw_path.to_string()
        };

        if path.starts_with("/public") {
            return self.serve_public_directory(session, &path).await;
        }

        let matched = if let Ok(m) = self.state.router.lookup(&path) {
            m
        } else {
            return self.renderer.handle_not_found_response(session).await;
        };

        let metadata = if let RouteKind::Page(page) = &matched.entry.kind {
            page.metadata.clone()
        } else {
            RouteMetadata::default()
        };

        let mut req = Request::new(
            session,
            self.state.user_state.clone(),
            // session.req_header(),
            matched.params,
            "req-id",
            metadata,
        );

        println!("{} {}", req.method(), path);

        if let Err(_) = self.run_middlewares(&mut req).await {
            return self.renderer.return_error_page(session).await;
        }

        match &matched.entry.kind {
            RouteKind::Page(page) => {
                if req.method() != http::Method::GET {
                    let page_ctr = page
                        .controllers
                        .get(req.method())
                        .ok_or_else(|| pingora::Error::new_str("405"))?;
                    let res = page_ctr.call_erased(req.clone()).await;
                    return self
                        .finalize_response(session, &req, Some(res), None)
                        .await;
                }
                self.renderer.render_page(session, &mut req, page).await
            }
            RouteKind::Api(api) => {
                let ctr = api
                    .controllers
                    .get(req.method())
                    .ok_or_else(|| pingora::Error::new_str("405"))?;
                let res = ctr.call_erased(req.clone()).await;
                self
                    .finalize_response(session, &req, Some(res), None)
                    .await
            }
        }
    }
}