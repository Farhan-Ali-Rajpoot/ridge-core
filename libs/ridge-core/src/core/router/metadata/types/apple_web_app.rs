use super::{TagDescriptor, TagProp};
use serde::Serialize;
use std::borrow::Cow;


#[derive(Clone, Debug, Default, Serialize)]
pub struct AppleWebApp {
    /// Enables standalone (full‑screen) mode. Values: "yes", "no".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capable: Option<Cow<'static, str>>,

    /// Sets the title on the home screen icon. If not provided, the page title is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'static, str>>,

    /// Status bar appearance. Values: "default", "black", "black-translucent".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_bar_style: Option<Cow<'static, str>>,
}

impl AppleWebApp {
    /// Renders HTML meta tags for Apple web app.
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(128);
        meta_tags!(html,
            ("meta", "name", "apple-mobile-web-app-capable", self.capable.as_deref()),
            ("meta", "name", "apple-mobile-web-app-title", self.title.as_deref()),
            ("meta", "name", "apple-mobile-web-app-status-bar-style", self.status_bar_style.as_deref()),
        );
        html
    }

    /// Collects flat tag descriptors for all Apple web app meta tags.
    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(capable) = &self.capable {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "apple-mobile-web-app-capable".to_string() },
                    TagProp { key: "content".to_string(), value: capable.to_string() },
                ],
            });
        }
        if let Some(title) = &self.title {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "apple-mobile-web-app-title".to_string() },
                    TagProp { key: "content".to_string(), value: title.to_string() },
                ],
            });
        }
        if let Some(style) = &self.status_bar_style {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "apple-mobile-web-app-status-bar-style".to_string() },
                    TagProp { key: "content".to_string(), value: style.to_string() },
                ],
            });
        }
    }

    /// Merges with parent: current fields override parent's if present, otherwise inherit from parent.
    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            capable: self.capable.clone().or_else(|| parent.capable.clone()),
            title: self.title.clone().or_else(|| parent.title.clone()),
            status_bar_style: self.status_bar_style.clone().or_else(|| parent.status_bar_style.clone()),
        }
    }
}