use super::{TagDescriptor, TagProp};
use const_format::formatcp;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OpenGraph {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_locales: Option<Vec<Cow<'static, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub determiner: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub og_type: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<OgImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<OgVideo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<OgAudio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<ArticleMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<BookMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileMetadata>,
}

impl OpenGraph {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(1024);

        meta_tags!(html,
            ("meta", "property", "og:title", self.title.as_deref()),
            ("meta", "property", "og:description", self.description.as_deref()),
            ("meta", "property", "og:url", self.url.as_deref()),
            ("meta", "property", "og:site_name", self.site_name.as_deref()),
            ("meta", "property", "og:locale", self.locale.as_deref()),
            ("meta", "property", "og:determiner", self.determiner.as_deref()),
            ("meta", "property", "og:type", self.og_type.as_deref()),
        );

        if let Some(locales) = &self.alternate_locales {
            for locale in locales {
                html.push_str(&format!("<meta property=\"og:locale:alternate\" content=\"{}\" />\n", locale.as_ref()));
            }
        }

        if let Some(images) = &self.images {
            for img in images { html.push_str(&img.render_html()); }
        }
        if let Some(videos) = &self.videos {
            for vid in videos { html.push_str(&vid.render_html()); }
        }
        if let Some(audio) = &self.audio {
            for aud in audio { html.push_str(&aud.render_html()); }
        }

        if let Some(ref article) = self.article { html.push_str(&article.render_html()); }
        if let Some(ref book) = self.book { html.push_str(&book.render_html()); }
        if let Some(ref profile) = self.profile { html.push_str(&profile.render_html()); }

        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Basic Open Graph meta tags
        if let Some(title) = &self.title {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:title".to_string() },
                    TagProp { key: "content".to_string(), value: title.to_string() },
                ],
            });
        }
        if let Some(description) = &self.description {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:description".to_string() },
                    TagProp { key: "content".to_string(), value: description.to_string() },
                ],
            });
        }
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:url".to_string() },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
        if let Some(site_name) = &self.site_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:site_name".to_string() },
                    TagProp { key: "content".to_string(), value: site_name.to_string() },
                ],
            });
        }
        if let Some(locale) = &self.locale {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:locale".to_string() },
                    TagProp { key: "content".to_string(), value: locale.to_string() },
                ],
            });
        }
        if let Some(determiner) = &self.determiner {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:determiner".to_string() },
                    TagProp { key: "content".to_string(), value: determiner.to_string() },
                ],
            });
        }
        if let Some(og_type) = &self.og_type {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:type".to_string() },
                    TagProp { key: "content".to_string(), value: og_type.to_string() },
                ],
            });
        }

        // Alternate locales
        if let Some(locales) = &self.alternate_locales {
            for locale in locales {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "property".to_string(), value: "og:locale:alternate".to_string() },
                        TagProp { key: "content".to_string(), value: locale.to_string() },
                    ],
                });
            }
        }

        // Images, videos, audio
        if let Some(images) = &self.images {
            for img in images {
                img.collect_tags(tags);
            }
        }
        if let Some(videos) = &self.videos {
            for vid in videos {
                vid.collect_tags(tags);
            }
        }
        if let Some(audio) = &self.audio {
            for aud in audio {
                aud.collect_tags(tags);
            }
        }

        // Type-specific metadata
        if let Some(article) = &self.article {
            article.collect_tags(tags);
        }
        if let Some(book) = &self.book {
            book.collect_tags(tags);
        }
        if let Some(profile) = &self.profile {
            profile.collect_tags(tags);
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            title: self.title.clone().or_else(|| parent.title.clone()),
            description: self.description.clone().or_else(|| parent.description.clone()),
            url: self.url.clone().or_else(|| parent.url.clone()),
            site_name: self.site_name.clone().or_else(|| parent.site_name.clone()),
            locale: self.locale.clone().or_else(|| parent.locale.clone()),
            alternate_locales: self.alternate_locales.clone().or_else(|| parent.alternate_locales.clone()),
            determiner: self.determiner.clone().or_else(|| parent.determiner.clone()),
            og_type: self.og_type.clone().or_else(|| parent.og_type.clone()),
            images: self.images.clone().or_else(|| parent.images.clone()),
            videos: self.videos.clone().or_else(|| parent.videos.clone()),
            audio: self.audio.clone().or_else(|| parent.audio.clone()),
            article: match (&self.article, &parent.article) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            book: match (&self.book, &parent.book) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            profile: match (&self.profile, &parent.profile) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OgImage {
    pub url: Cow<'static, str>,
    pub width: u32,
    pub height: u32,
    pub alt: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Cow<'static, str>>,
}

impl Default for OgImage {
    fn default() -> Self {
        Self {
            url: Cow::Borrowed(""),
            width: 0,
            height: 0,
            alt: Cow::Borrowed(""),
            secure_url: None,
            media_type: None,
        }
    }
}

impl OgImage {
    pub fn render_html(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!("<meta property=\"og:image\" content=\"{}\" />\n", self.url.as_ref()));

        let w = self.width.to_string();
        let h = self.height.to_string();

        meta_tags!(html,
            ("meta", "property", "og:image:width", Some(w.as_str())),
            ("meta", "property", "og:image:height", Some(h.as_str())),
            ("meta", "property", "og:image:alt", Some(self.alt.as_ref())),
            ("meta", "property", "og:image:secure_url", self.secure_url.as_deref()),
            ("meta", "property", "og:image:type", self.media_type.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Main image meta tag
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:image".to_string() },
                TagProp { key: "content".to_string(), value: self.url.to_string() },
            ],
        });

        // Width
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:image:width".to_string() },
                TagProp { key: "content".to_string(), value: self.width.to_string() },
            ],
        });

        // Height
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:image:height".to_string() },
                TagProp { key: "content".to_string(), value: self.height.to_string() },
            ],
        });

        // Alt
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:image:alt".to_string() },
                TagProp { key: "content".to_string(), value: self.alt.to_string() },
            ],
        });

        // Secure URL
        if let Some(secure_url) = &self.secure_url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:image:secure_url".to_string() },
                    TagProp { key: "content".to_string(), value: secure_url.to_string() },
                ],
            });
        }

        // Type
        if let Some(media_type) = &self.media_type {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:image:type".to_string() },
                    TagProp { key: "content".to_string(), value: media_type.to_string() },
                ],
            });
        }
    }

    // No inherit_from needed? It's not present, but if needed we could add.
    // However the user hasn't asked to add inherit_from for these.
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OgVideo {
    pub url: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Cow<'static, str>>,
}

impl OgVideo {
    pub fn render_html(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!("<meta property=\"og:video\" content=\"{}\" />\n", self.url.as_ref()));

        let w = self.width.map(|v| v.to_string());
        let h = self.height.map(|v| v.to_string());

        meta_tags!(html,
            ("meta", "property", "og:video:secure_url", self.secure_url.as_deref()),
            ("meta", "property", "og:video:type", self.media_type.as_deref()),
            ("meta", "property", "og:video:width", w.as_deref()),
            ("meta", "property", "og:video:height", h.as_deref()),
            ("meta", "property", "og:video:alt", self.alt.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Main video tag
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:video".to_string() },
                TagProp { key: "content".to_string(), value: self.url.to_string() },
            ],
        });

        if let Some(secure_url) = &self.secure_url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:video:secure_url".to_string() },
                    TagProp { key: "content".to_string(), value: secure_url.to_string() },
                ],
            });
        }

        if let Some(media_type) = &self.media_type {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:video:type".to_string() },
                    TagProp { key: "content".to_string(), value: media_type.to_string() },
                ],
            });
        }

        if let Some(width) = self.width {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:video:width".to_string() },
                    TagProp { key: "content".to_string(), value: width.to_string() },
                ],
            });
        }

        if let Some(height) = self.height {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:video:height".to_string() },
                    TagProp { key: "content".to_string(), value: height.to_string() },
                ],
            });
        }

        if let Some(alt) = &self.alt {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:video:alt".to_string() },
                    TagProp { key: "content".to_string(), value: alt.to_string() },
                ],
            });
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OgAudio {
    pub url: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Cow<'static, str>>,
}

impl OgAudio {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        html.push_str(&format!("<meta property=\"og:audio\" content=\"{}\" />\n", self.url.as_ref()));
        meta_tags!(html,
            ("meta", "property", "og:audio:secure_url", self.secure_url.as_deref()),
            ("meta", "property", "og:audio:type", self.media_type.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        tags.push(TagDescriptor {
            r#type: "meta".to_string(),
            content: None,
            props: vec![
                TagProp { key: "property".to_string(), value: "og:audio".to_string() },
                TagProp { key: "content".to_string(), value: self.url.to_string() },
            ],
        });
        if let Some(secure_url) = &self.secure_url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:audio:secure_url".to_string() },
                    TagProp { key: "content".to_string(), value: secure_url.to_string() },
                ],
            });
        }
        if let Some(media_type) = &self.media_type {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "og:audio:type".to_string() },
                    TagProp { key: "content".to_string(), value: media_type.to_string() },
                ],
            });
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_time: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Cow<'static, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Cow<'static, str>>>,
}

impl ArticleMetadata {
    pub fn render_html(&self) -> String {
        let mut html = String::new();
        meta_tags!(html,
            ("meta", "property", "article:published_time", self.published_time.as_deref()),
            ("meta", "property", "article:modified_time", self.modified_time.as_deref()),
            ("meta", "property", "article:expiration_time", self.expiration_time.as_deref()),
            ("meta", "property", "article:section", self.section.as_deref()),
        );

        if let Some(authors) = &self.authors {
            for author in authors {
                html.push_str(&format!("<meta property=\"article:author\" content=\"{}\" />\n", author.as_ref()));
            }
        }

        if let Some(tags) = &self.tags {
            for tag in tags {
                html.push_str(&format!("<meta property=\"article:tag\" content=\"{}\" />\n", tag.as_ref()));
            }
        }

        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(published_time) = &self.published_time {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "article:published_time".to_string() },
                    TagProp { key: "content".to_string(), value: published_time.to_string() },
                ],
            });
        }
        if let Some(modified_time) = &self.modified_time {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "article:modified_time".to_string() },
                    TagProp { key: "content".to_string(), value: modified_time.to_string() },
                ],
            });
        }
        if let Some(expiration_time) = &self.expiration_time {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "article:expiration_time".to_string() },
                    TagProp { key: "content".to_string(), value: expiration_time.to_string() },
                ],
            });
        }
        if let Some(authors) = &self.authors {
            for author in authors {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "property".to_string(), value: "article:author".to_string() },
                        TagProp { key: "content".to_string(), value: author.to_string() },
                    ],
                });
            }
        }
        if let Some(section) = &self.section {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "article:section".to_string() },
                    TagProp { key: "content".to_string(), value: section.to_string() },
                ],
            });
        }
        if let Some(tags_list) = &self.tags {
            for tag in tags_list {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "property".to_string(), value: "article:tag".to_string() },
                        TagProp { key: "content".to_string(), value: tag.to_string() },
                    ],
                });
            }
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            published_time: self.published_time.clone().or_else(|| parent.published_time.clone()),
            modified_time: self.modified_time.clone().or_else(|| parent.modified_time.clone()),
            expiration_time: self.expiration_time.clone().or_else(|| parent.expiration_time.clone()),
            authors: self.authors.clone().or_else(|| parent.authors.clone()),
            section: self.section.clone().or_else(|| parent.section.clone()),
            tags: self.tags.clone().or_else(|| parent.tags.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BookMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Cow<'static, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Cow<'static, str>>>,
}

impl BookMetadata {
    pub fn render_html(&self) -> String {
        let mut html = String::new();
        meta_tags!(html,
            ("meta", "property", "book:isbn", self.isbn.as_deref()),
            ("meta", "property", "book:release_date", self.release_date.as_deref()),
        );

        if let Some(tags) = &self.tags {
            for tag in tags {
                html.push_str(&format!("<meta property=\"book:tag\" content=\"{}\" />\n", tag.as_ref()));
            }
        }

        if let Some(authors) = &self.authors {
            for author in authors {
                html.push_str(&format!("<meta property=\"book:author\" content=\"{}\" />\n", author.as_ref()));
            }
        }

        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(isbn) = &self.isbn {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "book:isbn".to_string() },
                    TagProp { key: "content".to_string(), value: isbn.to_string() },
                ],
            });
        }
        if let Some(release_date) = &self.release_date {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "book:release_date".to_string() },
                    TagProp { key: "content".to_string(), value: release_date.to_string() },
                ],
            });
        }
        if let Some(tags_list) = &self.tags {
            for tag in tags_list {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "property".to_string(), value: "book:tag".to_string() },
                        TagProp { key: "content".to_string(), value: tag.to_string() },
                    ],
                });
            }
        }
        if let Some(authors_list) = &self.authors {
            for author in authors_list {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "property".to_string(), value: "book:author".to_string() },
                        TagProp { key: "content".to_string(), value: author.to_string() },
                    ],
                });
            }
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            isbn: self.isbn.clone().or_else(|| parent.isbn.clone()),
            release_date: self.release_date.clone().or_else(|| parent.release_date.clone()),
            tags: self.tags.clone().or_else(|| parent.tags.clone()),
            authors: self.authors.clone().or_else(|| parent.authors.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProfileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Cow<'static, str>>,
}

impl ProfileMetadata {
    pub fn render_html(&self) -> String {
        let mut html = String::new();
        meta_tags!(html,
            ("meta", "property", "profile:first_name", self.first_name.as_deref()),
            ("meta", "property", "profile:last_name", self.last_name.as_deref()),
            ("meta", "property", "profile:username", self.username.as_deref()),
            ("meta", "property", "profile:gender", self.gender.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(first_name) = &self.first_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "profile:first_name".to_string() },
                    TagProp { key: "content".to_string(), value: first_name.to_string() },
                ],
            });
        }
        if let Some(last_name) = &self.last_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "profile:last_name".to_string() },
                    TagProp { key: "content".to_string(), value: last_name.to_string() },
                ],
            });
        }
        if let Some(username) = &self.username {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "profile:username".to_string() },
                    TagProp { key: "content".to_string(), value: username.to_string() },
                ],
            });
        }
        if let Some(gender) = &self.gender {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "profile:gender".to_string() },
                    TagProp { key: "content".to_string(), value: gender.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            first_name: self.first_name.clone().or_else(|| parent.first_name.clone()),
            last_name: self.last_name.clone().or_else(|| parent.last_name.clone()),
            username: self.username.clone().or_else(|| parent.username.clone()),
            gender: self.gender.clone().or_else(|| parent.gender.clone()),
        }
    }
}