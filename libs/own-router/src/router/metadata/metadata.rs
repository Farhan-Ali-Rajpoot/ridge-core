use super::{
    AppleWebApp, Icon, Author, TwitterCard, OpenGraph, Robots, Alternates, ReferrerPolicy, MaxImagePreview,
    MediaAlternate, TypeAlternate, AppLinks, Verification, MetaTag, LinkTag, LanguageAlternate,

    TagDescriptor, TagProp,
};
use const_format::formatcp;
use serde::Serialize;
use std::borrow::Cow;


#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteMetadata {
    // Core
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<Cow<'static, str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<ReferrerPolicy>,

    // Robots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Robots>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub googlebot: Option<Robots>,

    // Canonical & Alternates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternates: Option<Alternates>,

    // Open Graph
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_graph: Option<OpenGraph>,

    // Twitter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<TwitterCard>,

    // Icons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<Icon>>,

    // PWA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<Cow<'static, str>>,

    // Apple Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_web_app: Option<AppleWebApp>,

    // App Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_links: Option<AppLinks>,

    // Verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,

    // Structured Data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_ld: Option<Cow<'static, str>>,

    // Custom Meta / Link (always present, default to empty)
    #[serde(default)]
    pub custom_meta: Vec<MetaTag>,
    #[serde(default)]
    pub custom_links: Vec<LinkTag>,

    // App Behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_hint: Option<Cow<'static, str>>,
    pub requires_auth: bool,
    pub skip_layouts: bool,
}

impl RouteMetadata {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(2048);

        // Title
        if let Some(title) = &self.title {
            html.push_str(&format!("<title>{}</title>\n", title.as_ref()));
        }

        // Core meta tags
        meta_tags!(html,
            ("meta", "name", "description", self.description.as_deref()),
            ("meta", "name", "creator", self.creator.as_deref()),
            ("meta", "name", "publisher", self.publisher.as_deref()),
            ("meta", "name", "category", self.category.as_deref()),
            ("meta", "name", "classification", self.classification.as_deref()),
        );

        // Keywords
        if let Some(keywords) = &self.keywords {
            let kw_str: Vec<&str> = keywords.iter().map(|k| k.as_ref()).collect();
            html.push_str(&format!("<meta name=\"keywords\" content=\"{}\" />\n", kw_str.join(", ")));
        }

        // Authors
        if let Some(authors) = &self.authors {
            for author in authors {
                html.push_str(&author.render_html());
            }
        }

        // Referrer policy
        if let Some(ref rp) = self.referrer {
            html.push_str(&format!("<meta name=\"referrer\" content=\"{}\" />\n", rp.as_str()));
        }

        // Robots
        if let Some(robots) = &self.robots {
            html.push_str(&robots.render_html());
        }
        if let Some(googlebot) = &self.googlebot {
            html.push_str(&googlebot.render_html().replace("robots", "googlebot"));
        }

        // Canonical & Alternates
        if let Some(alt) = &self.alternates {
            html.push_str(&alt.render_html());
        }

        // Open Graph
        if let Some(og) = &self.open_graph {
            html.push_str(&og.render_html());
        }

        // Twitter
        if let Some(tw) = &self.twitter {
            html.push_str(&tw.render_html());
        }

        // Icons
        if let Some(icons) = &self.icons {
            for icon in icons {
                html.push_str(&icon.render_html());
            }
        }

        // PWA
        if let Some(manifest) = &self.manifest {
            html.push_str(&format!("<link rel=\"manifest\" href=\"{}\" />\n", manifest.as_ref()));
        }
        if let Some(theme) = &self.theme_color {
            html.push_str(&format!("<meta name=\"theme-color\" content=\"{}\" />\n", theme.as_ref()));
        }
        if let Some(scheme) = &self.color_scheme {
            html.push_str(&format!("<meta name=\"color-scheme\" content=\"{}\" />\n", scheme.as_ref()));
        }

        // Apple Web App
        if let Some(apple) = &self.apple_web_app {
            html.push_str(&apple.render_html());
        }

        // App Links
        if let Some(app) = &self.app_links {
            html.push_str(&app.render_html());
        }

        // Verification
        if let Some(ver) = &self.verification {
            html.push_str(&ver.render_html());
        }

        // Structured Data (JSON-LD)
        if let Some(json) = &self.json_ld {
            html.push_str("<script type=\"application/ld+json\">");
            html.push_str(json.as_ref());
            html.push_str("</script>\n");
        }

        // Custom meta tags
        for meta in &self.custom_meta {
            html.push_str(&meta.render_html());
        }

        // Custom link tags
        for link in &self.custom_links {
            html.push_str(&link.render_html());
        }

        // Stack hint (app behavior)
        if let Some(hint) = &self.stack_hint {
            html.push_str(&format!("<meta name=\"stack-hint\" content=\"{}\" />\n", hint.as_ref()));
        }

        // requires_auth and skip_layouts are not directly output in HTML;
        // they are used by the application logic.

        html
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}



impl RouteMetadata {

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            // Core
            title: self.title.clone().or_else(|| parent.title.clone()),
            description: self.description.clone().or_else(|| parent.description.clone()),
            keywords: self.keywords.clone().or_else(|| parent.keywords.clone()),
            authors: self.authors.clone().or_else(|| parent.authors.clone()),
            creator: self.creator.clone().or_else(|| parent.creator.clone()),
            publisher: self.publisher.clone().or_else(|| parent.publisher.clone()),
            category: self.category.clone().or_else(|| parent.category.clone()),
            classification: self.classification.clone().or_else(|| parent.classification.clone()),
            referrer: self.referrer.clone().or(parent.referrer.clone()),
        
            // Robots
            robots: match (&self.robots, &parent.robots) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            googlebot: match (&self.googlebot, &parent.googlebot) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Canonical & Alternates
            alternates: match (&self.alternates, &parent.alternates) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Open Graph
            open_graph: match (&self.open_graph, &parent.open_graph) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Twitter
            twitter: match (&self.twitter, &parent.twitter) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Icons
            icons: self.icons.clone().or_else(|| parent.icons.clone()),
        
            // PWA
            manifest: self.manifest.clone().or_else(|| parent.manifest.clone()),
            theme_color: self.theme_color.clone().or_else(|| parent.theme_color.clone()),
            color_scheme: self.color_scheme.clone().or_else(|| parent.color_scheme.clone()),
        
            // Apple Web App
            apple_web_app: match (&self.apple_web_app, &parent.apple_web_app) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // App Links
            app_links: match (&self.app_links, &parent.app_links) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Verification
            verification: match (&self.verification, &parent.verification) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Structured Data
            json_ld: self.json_ld.clone().or_else(|| parent.json_ld.clone()),
        
            // Custom Meta / Link
            custom_meta: {
                let mut merged = Vec::new();
                let parent_map: std::collections::HashMap<_, &MetaTag> = parent.custom_meta
                    .iter()
                    .filter_map(|tag| tag.key().map(|k| (k, tag)))
                    .collect();
            
                for child in &self.custom_meta {
                    if let Some(key) = child.key() {
                        if let Some(parent_tag) = parent_map.get(&key) {
                            merged.push(child.inherit_from(parent_tag));
                        } else {
                            merged.push(child.clone());
                        }
                    } else {
                        merged.push(child.clone());
                    }
                }
            
                for parent_tag in &parent.custom_meta {
                    if let Some(key) = parent_tag.key() {
                        if !self.custom_meta.iter().any(|child| child.key().as_ref() == Some(&key)) {
                            merged.push(parent_tag.clone());
                        }
                    } else if !self.custom_meta.contains(parent_tag) {
                        merged.push(parent_tag.clone());
                    }
                }
                merged
            },
        
            custom_links: {
                let mut merged = Vec::new();
                let parent_map: std::collections::HashMap<_, &LinkTag> = parent.custom_links
                    .iter()
                    .filter_map(|tag| tag.key().map(|k| (k, tag)))
                    .collect();
            
                for child in &self.custom_links {
                    if let Some(key) = child.key() {
                        if let Some(parent_tag) = parent_map.get(&key) {
                            merged.push(child.inherit_from(parent_tag));
                        } else {
                            merged.push(child.clone());
                        }
                    } else {
                        merged.push(child.clone());
                    }
                }
            
                for parent_tag in &parent.custom_links {
                    if let Some(key) = parent_tag.key() {
                        if !self.custom_links.iter().any(|child| child.key().as_ref() == Some(&key)) {
                            merged.push(parent_tag.clone());
                        }
                    } else if !self.custom_links.contains(parent_tag) {
                        merged.push(parent_tag.clone());
                    }
                }
                merged
            },
        
            // App Behavior
            stack_hint: self.stack_hint.clone().or_else(|| parent.stack_hint.clone()),
            requires_auth: self.requires_auth,
            skip_layouts: self.skip_layouts,
        }
    }

    pub fn collect_tags(&self) -> Vec<TagDescriptor> {
        let mut tags = Vec::new();

        // Title
        if let Some(title) = &self.title {
            tags.push(TagDescriptor {
                r#type: "title".to_string(),
                content: Some(title.to_string()),
                props: Vec::new(),
            });
        }

        // Core meta tags
        if let Some(desc) = &self.description {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "description".to_string() },
                    TagProp { key: "content".to_string(), value: desc.to_string() },
                ],
            });
        }
        if let Some(creator) = &self.creator {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "creator".to_string() },
                    TagProp { key: "content".to_string(), value: creator.to_string() },
                ],
            });
        }
        if let Some(publisher) = &self.publisher {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "publisher".to_string() },
                    TagProp { key: "content".to_string(), value: publisher.to_string() },
                ],
            });
        }
        if let Some(category) = &self.category {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "category".to_string() },
                    TagProp { key: "content".to_string(), value: category.to_string() },
                ],
            });
        }
        if let Some(classification) = &self.classification {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "classification".to_string() },
                    TagProp { key: "content".to_string(), value: classification.to_string() },
                ],
            });
        }

        // Keywords
        if let Some(keywords) = &self.keywords {
            let kw_str: Vec<String> = keywords.iter().map(|k| k.to_string()).collect();
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "keywords".to_string() },
                    TagProp { key: "content".to_string(), value: kw_str.join(", ") },
                ],
            });
        }

        // Authors (each pushes its own tags)
        if let Some(authors) = &self.authors {
            for author in authors {
                author.collect_tags(&mut tags);
            }
        }

        // Referrer policy
        if let Some(rp) = &self.referrer {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "referrer".to_string() },
                    TagProp { key: "content".to_string(), value: rp.as_str().to_string() },
                ],
            });
        }

        // Robots
        if let Some(robots) = &self.robots {
            robots.collect_tags(&mut tags);
        }
        if let Some(googlebot) = &self.googlebot {
            // We need to collect googlebot tags; robots.collect_tags uses name="robots".
            // We'll create a separate tag descriptor for googlebot by reusing the same directives but with name="googlebot".
            let mut directives = Vec::new();
            // Replicate the logic from Robots::render_html to build directives string.
            if let Some(val) = googlebot.index {
                directives.push(if val { "index" } else { "noindex" }.to_string());
            }
            if let Some(val) = googlebot.follow {
                directives.push(if val { "follow" } else { "nofollow" }.to_string());
            }
            if googlebot.noarchive == Some(true) {
                directives.push("noarchive".to_string());
            }
            if googlebot.nosnippet == Some(true) {
                directives.push("nosnippet".to_string());
            }
            if let Some(val) = googlebot.max_snippet {
                directives.push(format!("max-snippet:{}", val));
            }
            if let Some(val) = &googlebot.max_image_preview {
                directives.push(format!("max-image-preview:{}", val.as_str()));
            }
            if let Some(val) = googlebot.max_video_preview {
                directives.push(format!("max-video-preview:{}", val));
            }
            if googlebot.notranslate == Some(true) {
                directives.push("notranslate".to_string());
            }
            if googlebot.noimageindex == Some(true) {
                directives.push("noimageindex".to_string());
            }
            if let Some(val) = &googlebot.unavailable_after {
                directives.push(format!("unavailable_after:{}", val.as_ref()));
            }
            if !directives.is_empty() {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "name".to_string(), value: "googlebot".to_string() },
                        TagProp { key: "content".to_string(), value: directives.join(", ") },
                    ],
                });
            }
        }

        // Alternates
        if let Some(alt) = &self.alternates {
            alt.collect_tags(&mut tags);
        }

        // Open Graph
        if let Some(og) = &self.open_graph {
            og.collect_tags(&mut tags);
        }

        // Twitter
        if let Some(tw) = &self.twitter {
            tw.collect_tags(&mut tags);
        }

        // Icons
        if let Some(icons) = &self.icons {
            for icon in icons {
                icon.collect_tags(&mut tags);
            }
        }

        // PWA
        if let Some(manifest) = &self.manifest {
            tags.push(TagDescriptor {
                r#type: "link".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "rel".to_string(), value: "manifest".to_string() },
                    TagProp { key: "href".to_string(), value: manifest.to_string() },
                ],
            });
        }
        if let Some(theme) = &self.theme_color {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "theme-color".to_string() },
                    TagProp { key: "content".to_string(), value: theme.to_string() },
                ],
            });
        }
        if let Some(scheme) = &self.color_scheme {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "color-scheme".to_string() },
                    TagProp { key: "content".to_string(), value: scheme.to_string() },
                ],
            });
        }

        // Apple Web App
        if let Some(apple) = &self.apple_web_app {
            apple.collect_tags(&mut tags);
        }

        // App Links
        if let Some(app) = &self.app_links {
            app.collect_tags(&mut tags);
        }

        // Verification
        if let Some(ver) = &self.verification {
            ver.collect_tags(&mut tags);
        }

        // Structured Data (JSON-LD) – special, content is the JSON string
        if let Some(json) = &self.json_ld {
            tags.push(TagDescriptor {
                r#type: "script".to_string(),
                content: Some(json.to_string()),
                props: vec![
                    TagProp { key: "type".to_string(), value: "application/ld+json".to_string() },
                ],
            });
        }

        // Custom meta tags
        for meta in &self.custom_meta {
            meta.collect_tags(&mut tags);
        }

        // Custom link tags
        for link in &self.custom_links {
            link.collect_tags(&mut tags);
        }

        // Stack hint (app behavior)
        if let Some(hint) = &self.stack_hint {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "stack-hint".to_string() },
                    TagProp { key: "content".to_string(), value: hint.to_string() },
                ],
            });
        }

        tags
    }

    pub fn to_flat_json(&self) -> serde_json::Value {
        let tags = self.collect_tags();
        serde_json::to_value(tags).unwrap_or(serde_json::Value::Null)
    }
}