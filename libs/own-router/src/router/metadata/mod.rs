#[macro_use]
pub mod macros;

pub mod open_graph;
pub mod twitter_card;
pub mod metadata;
pub mod author;
pub mod robots;
pub mod alternates;
pub mod policies;
pub mod icons;
pub mod apple_web_app;
pub mod app_links;
pub mod verification;
pub mod custom_tag;

pub mod flat;

pub use open_graph::{OpenGraph, OgImage ,OgVideo, OgAudio, ArticleMetadata, BookMetadata, ProfileMetadata, };
pub use twitter_card::{TwitterCard, TwitterImage, TwitterApp, TwitterAppPlatform, TwitterPlayer, };
pub use metadata::{RouteMetadata};
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


use serde::Serialize;
use serde_json::{Value, json};

pub trait ProcessMetadata: Serialize {
    fn render_html(&self) -> String;

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}