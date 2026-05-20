pub mod collections;
mod config;
mod core;
mod error;
mod types;

pub use include_dir;
pub use rscx;

pub mod router {
    pub use crate::core::router::Router;
    pub use crate::core::router::handlers::LayoutProps;
    pub use crate::core::router::handlers::RenderedParallelRoute;
    pub use crate::core::router::logic::RouteMetadata;
}

pub mod request {
    pub use crate::core::router::io::request::Request;
    pub mod kinds {
        pub use crate::core::router::io::request::kinds::{Api, Page};
    }
}

pub mod response {
    pub use crate::core::router::io::response::Body;
    pub use crate::core::router::io::response::IntoResponse;
    pub use crate::core::router::io::response::Response;
}

pub mod builder {
    pub use crate::core::builder::{AppBuilder, Config};
    pub use crate::core::pingora::renderer::Renderer;
}
