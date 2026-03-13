use super::{TagDescriptor, TagProp};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<TwitterImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<TwitterPlayer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<TwitterApp>,
}

impl TwitterCard {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(1024);
        meta_tags!(html,
            ("meta", "name", "twitter:card", self.card.as_deref()),
            ("meta", "name", "twitter:site", self.site.as_deref()),
            ("meta", "name", "twitter:site:id", self.site_id.as_deref()),
            ("meta", "name", "twitter:creator", self.creator.as_deref()),
            ("meta", "name", "twitter:creator:id", self.creator_id.as_deref()),
            ("meta", "name", "twitter:title", self.title.as_deref()),
            ("meta", "name", "twitter:description", self.description.as_deref()),
        );
        if let Some(ref img) = self.image {
            html.push_str(&img.render_html());
        }
        if let Some(ref player) = self.player {
            html.push_str(&player.render_html());
        }
        if let Some(ref app) = self.app {
            html.push_str(&app.render_html());
        }
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Basic twitter meta tags
        if let Some(card) = &self.card {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:card".to_string() },
                    TagProp { key: "content".to_string(), value: card.to_string() },
                ],
            });
        }
        if let Some(site) = &self.site {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:site".to_string() },
                    TagProp { key: "content".to_string(), value: site.to_string() },
                ],
            });
        }
        if let Some(site_id) = &self.site_id {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:site:id".to_string() },
                    TagProp { key: "content".to_string(), value: site_id.to_string() },
                ],
            });
        }
        if let Some(creator) = &self.creator {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:creator".to_string() },
                    TagProp { key: "content".to_string(), value: creator.to_string() },
                ],
            });
        }
        if let Some(creator_id) = &self.creator_id {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:creator:id".to_string() },
                    TagProp { key: "content".to_string(), value: creator_id.to_string() },
                ],
            });
        }
        if let Some(title) = &self.title {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:title".to_string() },
                    TagProp { key: "content".to_string(), value: title.to_string() },
                ],
            });
        }
        if let Some(desc) = &self.description {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:description".to_string() },
                    TagProp { key: "content".to_string(), value: desc.to_string() },
                ],
            });
        }
        // Nested structs
        if let Some(img) = &self.image {
            img.collect_tags(tags);
        }
        if let Some(player) = &self.player {
            player.collect_tags(tags);
        }
        if let Some(app) = &self.app {
            app.collect_tags(tags);
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            card: self.card.clone().or_else(|| parent.card.clone()),
            site: self.site.clone().or_else(|| parent.site.clone()),
            site_id: self.site_id.clone().or_else(|| parent.site_id.clone()),
            creator: self.creator.clone().or_else(|| parent.creator.clone()),
            creator_id: self.creator_id.clone().or_else(|| parent.creator_id.clone()),
            title: self.title.clone().or_else(|| parent.title.clone()),
            description: self.description.clone().or_else(|| parent.description.clone()),
            image: match (&self.image, &parent.image) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            player: match (&self.player, &parent.player) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            app: match (&self.app, &parent.app) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterImage {
    pub url: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Cow<'static, str>>,
}

impl TwitterImage {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(128);
        html.push_str(&format!("<meta name=\"twitter:image\" content=\"{}\" />\n", self.url.as_ref()));
        if let Some(ref alt) = self.alt {
            html.push_str(&format!("<meta name=\"twitter:image:alt\" content=\"{}\" />\n", alt.as_ref()));
        }
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "name".to_string(), value: "twitter:image".to_string() },
                TagProp { key: "content".to_string(), value: self.url.to_string() },
            ],
        });
        if let Some(alt) = &self.alt {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:image:alt".to_string() },
                    TagProp { key: "content".to_string(), value: alt.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone(),
            alt: self.alt.clone().or_else(|| parent.alt.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterPlayer {
    pub url: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Cow<'static, str>>,
}

impl TwitterPlayer {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        html.push_str(&format!("<meta name=\"twitter:player\" content=\"{}\" />\n", self.url.as_ref()));
        meta_tags!(html,
            ("meta", "name", "twitter:player:width", self.width.as_deref()),
            ("meta", "name", "twitter:player:height", self.height.as_deref()),
            ("meta", "name", "twitter:stream", self.stream.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "name".to_string(), value: "twitter:player".to_string() },
                TagProp { key: "content".to_string(), value: self.url.to_string() },
            ],
        });
        if let Some(width) = &self.width {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:player:width".to_string() },
                    TagProp { key: "content".to_string(), value: width.to_string() },
                ],
            });
        }
        if let Some(height) = &self.height {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:player:height".to_string() },
                    TagProp { key: "content".to_string(), value: height.to_string() },
                ],
            });
        }
        if let Some(stream) = &self.stream {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "twitter:stream".to_string() },
                    TagProp { key: "content".to_string(), value: stream.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone(),
            width: self.width.clone().or_else(|| parent.width.clone()),
            height: self.height.clone().or_else(|| parent.height.clone()),
            stream: self.stream.clone().or_else(|| parent.stream.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iphone: Option<TwitterAppPlatform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipad: Option<TwitterAppPlatform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub googleplay: Option<TwitterAppPlatform>,
}

impl TwitterApp {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(512);
        if let Some(ref platform) = self.iphone {
            render_platform(&mut html, "iphone", platform);
        }
        if let Some(ref platform) = self.ipad {
            render_platform(&mut html, "ipad", platform);
        }
        if let Some(ref platform) = self.googleplay {
            render_platform(&mut html, "googleplay", platform);
        }
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(iphone) = &self.iphone {
            iphone.collect_tags(tags, "iphone");
        }
        if let Some(ipad) = &self.ipad {
            ipad.collect_tags(tags, "ipad");
        }
        if let Some(googleplay) = &self.googleplay {
            googleplay.collect_tags(tags, "googleplay");
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            iphone: match (&self.iphone, &parent.iphone) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            ipad: match (&self.ipad, &parent.ipad) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            googleplay: match (&self.googleplay, &parent.googleplay) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TwitterAppPlatform {
    pub name: Cow<'static, str>,
    pub id: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
}

impl TwitterAppPlatform {
    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>, platform: &str) {
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "name".to_string(), value: format!("twitter:app:name:{}", platform) },
                TagProp { key: "content".to_string(), value: self.name.to_string() },
            ],
        });
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "name".to_string(), value: format!("twitter:app:id:{}", platform) },
                TagProp { key: "content".to_string(), value: self.id.to_string() },
            ],
        });
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: format!("twitter:app:url:{}", platform) },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            name: self.name.clone(),
            id: self.id.clone(),
            url: self.url.clone().or_else(|| parent.url.clone()),
        }
    }
}

// Keep the helper function render_platform if used elsewhere, but we can remove it if not needed
fn render_platform(html: &mut String, platform: &str, data: &TwitterAppPlatform) {
    html.push_str(&format!("<meta name=\"twitter:app:name:{}\" content=\"{}\" />\n", platform, data.name.as_ref()));
    html.push_str(&format!("<meta name=\"twitter:app:id:{}\" content=\"{}\" />\n", platform, data.id.as_ref()));
    if let Some(ref url) = data.url {
        html.push_str(&format!("<meta name=\"twitter:app:url:{}\" content=\"{}\" />\n", platform, url.as_ref()));
    }
}