use super::{TagDescriptor, TagProp}; 
use serde::{Serialize};
use std::borrow::{Cow};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Alternates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<LanguageAlternate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<MediaAlternate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<TypeAlternate>>,
}

impl Alternates {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(1024);

        meta_tags!(link: html,
            ("link", "rel", "canonical", self.canonical.as_deref()),
        );

        if let Some(ref langs) = self.languages {
            for item in langs {
                html.push_str(&item.render_html());
            }
        }

        if let Some(ref media_list) = self.media {
            for item in media_list {
                html.push_str(&item.render_html());
            }
        }

        if let Some(ref types_list) = self.types {
            for item in types_list {
                html.push_str(&item.render_html());
            }
        }

        html
    }

    /// Collects flat tag descriptors for all alternates.
    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Canonical link
        if let Some(canonical) = &self.canonical {
            tags.push(TagDescriptor {
                r#type: "link".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "rel".to_string(), value: "canonical".to_string() },
                    TagProp { key: "href".to_string(), value: canonical.to_string() },
                ],
            });
        }

        // Language alternates
        if let Some(langs) = &self.languages {
            for item in langs {
                item.collect_tags(tags);
            }
        }

        // Media alternates
        if let Some(media_list) = &self.media {
            for item in media_list {
                item.collect_tags(tags);
            }
        }

        // Type alternates
        if let Some(types_list) = &self.types {
            for item in types_list {
                item.collect_tags(tags);
            }
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            canonical: self.canonical.clone().or_else(|| parent.canonical.clone()),
            languages: self.languages.clone().or_else(|| parent.languages.clone()),
            media: self.media.clone().or_else(|| parent.media.clone()),
            types: self.types.clone().or_else(|| parent.types.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct LanguageAlternate {
    pub href_lang: Cow<'static, str>,
    pub href: Cow<'static, str>,
}

impl LanguageAlternate {
    pub fn render_html(&self) -> String {
        format!("<link rel=\"alternate\" hreflang=\"{}\" href=\"{}\" />\n", self.href_lang, self.href)
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "link".to_string(),
            content: None,
            props: vec![
                TagProp { key: "rel".to_string(), value: "alternate".to_string() },
                TagProp { key: "hreflang".to_string(), value: self.href_lang.to_string() },
                TagProp { key: "href".to_string(), value: self.href.to_string() },
            ],
        });
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MediaAlternate {
    pub media: Cow<'static, str>,
    pub href: Cow<'static, str>,
}

impl MediaAlternate {
    pub fn render_html(&self) -> String {
        format!("<link rel=\"alternate\" media=\"{}\" href=\"{}\" />\n", self.media, self.href)
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "link".to_string(),
            content: None,
            props: vec![
                TagProp { key: "rel".to_string(), value: "alternate".to_string() },
                TagProp { key: "media".to_string(), value: self.media.to_string() },
                TagProp { key: "href".to_string(), value: self.href.to_string() },
            ],
        });
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TypeAlternate {
    pub type_: Cow<'static, str>,
    pub href: Cow<'static, str>,
}

impl TypeAlternate {
    pub fn render_html(&self) -> String {
        format!("<link rel=\"alternate\" type=\"{}\" href=\"{}\" />\n", self.type_, self.href)
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "link".to_string(),
            content: None,
            props: vec![
                TagProp { key: "rel".to_string(), value: "alternate".to_string() },
                TagProp { key: "type".to_string(), value: self.type_.to_string() },
                TagProp { key: "href".to_string(), value: self.href.to_string() },
            ],
        });
    }
}