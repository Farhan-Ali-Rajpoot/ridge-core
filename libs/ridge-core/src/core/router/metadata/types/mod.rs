pub mod open_graph;
pub mod twitter_card;
pub mod author;
pub mod robots;
pub mod alternates;
pub mod policies;
pub mod icons;
pub mod apple_web_app;
pub mod app_links;
pub mod verification;
pub mod custom_tag;


pub use open_graph::{OpenGraph, OgImage ,OgVideo, OgAudio, ArticleMetadata, BookMetadata, ProfileMetadata, };
pub use twitter_card::{TwitterCard, TwitterImage, TwitterApp, TwitterAppPlatform, TwitterPlayer, };
pub use author::{Author};
pub use robots::{Robots, MaxImagePreview};
pub use alternates::{Alternates, TypeAlternate, MediaAlternate, LanguageAlternate};
pub use policies::{ReferrerPolicy,};
pub use icons::{Icon};
pub use apple_web_app::{AppleWebApp};
pub use app_links::{AppLinks, IosAppLink, AndroidAppLink, WindowsAppLink, WebAppLink};
pub use verification::{Verification};
pub use custom_tag::{MetaTag, LinkTag};
pub use flat::{TagProp, TagDescriptor};

pub use crate::core::router::metadata::core::{RouteMetadata};
pub use crate::core::router::metadata::flat;
