use super::{TagDescriptor, TagProp};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Robots {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noarchive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosnippet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_snippet: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_preview: Option<MaxImagePreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_preview: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notranslate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noimageindex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_after: Option<Cow<'static, str>>,
}

impl Robots {
    /// Renders the robots meta tag with all active directives.
    pub fn render_html(&self) -> String {
        let mut directives = Vec::new();

        if let Some(val) = self.index {
            directives.push(if val { "index" } else { "noindex" }.to_string());
        }
        if let Some(val) = self.follow {
            directives.push(if val { "follow" } else { "nofollow" }.to_string());
        }
        if self.noarchive == Some(true) {
            directives.push("noarchive".to_string());
        }
        if self.nosnippet == Some(true) {
            directives.push("nosnippet".to_string());
        }
        if let Some(val) = self.max_snippet {
            directives.push(format!("max-snippet:{}", val));
        }
        if let Some(val) = &self.max_image_preview {
            directives.push(format!("max-image-preview:{}", val.as_str()));
        }
        if let Some(val) = self.max_video_preview {
            directives.push(format!("max-video-preview:{}", val));
        }
        if self.notranslate == Some(true) {
            directives.push("notranslate".to_string());
        }
        if self.noimageindex == Some(true) {
            directives.push("noimageindex".to_string());
        }
        if let Some(val) = &self.unavailable_after {
            directives.push(format!("unavailable_after:{}", val.as_ref()));
        }

        if directives.is_empty() {
            return String::new();
        }

        format!("<meta name=\"robots\" content=\"{}\" />\n", directives.join(", "))
    }

    /// Collects flat tag descriptor for robots meta tag.
    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        let mut directives = Vec::new();

        if let Some(val) = self.index {
            directives.push(if val { "index" } else { "noindex" }.to_string());
        }
        if let Some(val) = self.follow {
            directives.push(if val { "follow" } else { "nofollow" }.to_string());
        }
        if self.noarchive == Some(true) {
            directives.push("noarchive".to_string());
        }
        if self.nosnippet == Some(true) {
            directives.push("nosnippet".to_string());
        }
        if let Some(val) = self.max_snippet {
            directives.push(format!("max-snippet:{}", val));
        }
        if let Some(val) = &self.max_image_preview {
            directives.push(format!("max-image-preview:{}", val.as_str()));
        }
        if let Some(val) = self.max_video_preview {
            directives.push(format!("max-video-preview:{}", val));
        }
        if self.notranslate == Some(true) {
            directives.push("notranslate".to_string());
        }
        if self.noimageindex == Some(true) {
            directives.push("noimageindex".to_string());
        }
        if let Some(val) = &self.unavailable_after {
            directives.push(format!("unavailable_after:{}", val.as_ref()));
        }

        if directives.is_empty() {
            return;
        }

        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "name".to_string(), value: "robots".to_string() },
                TagProp { key: "content".to_string(), value: directives.join(", ") },
            ],
        });
    }

    /// Merges with parent: current fields override parent's if present, otherwise inherit from parent.
    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            index: self.index.or(parent.index),
            follow: self.follow.or(parent.follow),
            noarchive: self.noarchive.or(parent.noarchive),
            nosnippet: self.nosnippet.or(parent.nosnippet),
            max_snippet: self.max_snippet.or(parent.max_snippet),
            max_image_preview: self.max_image_preview.or(parent.max_image_preview),
            max_video_preview: self.max_video_preview.or(parent.max_video_preview),
            notranslate: self.notranslate.or(parent.notranslate),
            noimageindex: self.noimageindex.or(parent.noimageindex),
            unavailable_after: self.unavailable_after.clone().or_else(|| parent.unavailable_after.clone()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum MaxImagePreview {
    #[default]
    Standard,
    Large,
}

impl MaxImagePreview {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaxImagePreview::Standard => "standard",
            MaxImagePreview::Large => "large",
        }
    }
}