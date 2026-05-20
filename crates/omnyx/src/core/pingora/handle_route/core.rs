use pingora::proxy::Session;

use crate::core::pingora::PingoraAdapter;
use crate::core::router::io::request::{Connection, Request, kinds::{Page, Api}};
use crate::core::{RouteKind, RouteMetadata};

impl<T> PingoraAdapter<T>
where
    T: Send + Sync + 'static,
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

        let mut connection =
            Connection::new(self.state.user_state.clone(), session, matched.params);

        if let Err(_) = self.run_middlewares(&mut connection).await {
            return self.renderer.return_error_page(session).await;
        }

        // println!("{} {}", req.method(), path);

        match &matched.entry.kind {
            RouteKind::Page(page) => {
                let mut req = Request::<Page>::new( Page::new(connection, metadata) );

                if req.method() != http::Method::GET {
                    let page_ctr = page
                        .controllers
                        .get(req.method())
                        .ok_or_else(|| pingora::Error::new_str("405"))?;
                    let res = page_ctr.call_erased(req.clone()).await;
                    return self.renderer.finalize_streaming_response(session, &req, Some(res), None).await;
                }
                self.renderer.render_page(session, &mut req, page).await
            }
            RouteKind::Api(api) => {
                let req = Request::<Api>::new( Api::new(connection) );

                let ctr = api
                    .controllers
                    .get(req.method())
                    .ok_or_else(|| pingora::Error::new_str("405"))?;
                let res = {
                    // 1. Move the cloned request directly into the controller's execution space
                    let owned_req = req.clone();
                    
                    // 2. Erase any other local references to `req` in this sub-scope block
                    // by mapping the future execution directly
                    ctr.call_erased(owned_req)
                }.await;
                self.finalize_response(session, &req, Some(res)).await
            }
        }
    }
}
