use serde::{Serialize, Deserialize};

use super::core::{RouteMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagProp {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagDescriptor {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub props: Vec<TagProp>,
}


impl RouteMetadata {

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