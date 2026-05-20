use std::sync::Arc;

use crate::core::router::handlers::{ErasedLayoutComponent, LayoutComponentWrapper, LayoutProps};
use crate::core::router::io::request::{Request, Page};

use super::templates::{NOT_FOUND_PAGE, ERROR_PAGE};




pub struct FrameworkFallbacks {
    pub(crate) not_found_html: &'static str,
    pub(crate) error_html: &'static str,
    pub(crate) root_layout: Arc<dyn ErasedLayoutComponent>,
}

impl Default for FrameworkFallbacks {
    fn default() -> Self {
        let default_root = Arc::new(LayoutComponentWrapper {
            handler: async move |req: Request<Page>, props: LayoutProps| {
                rscx::html! {
                    <!DOCTYPE html>
                    <html lang="en">
                        <head>
                            { &req.metadata().render_html() }
                            <meta charset="utf-8" />
                            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                        </head>
                        <body>
                            { props.children }
                        </body>
                    </html>
                }
            },
            _marker: std::marker::PhantomData,
        });

        Self {
            not_found_html: NOT_FOUND_PAGE,
            error_html: ERROR_PAGE,
            root_layout: default_root,
        }
    }
}