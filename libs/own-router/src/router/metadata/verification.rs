use super::{TagDescriptor, TagProp};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Verification {
    /// Google Search Console verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google: Option<Cow<'static, str>>,

    /// Yandex Webmaster verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yandex: Option<Cow<'static, str>>,

    /// Bing Webmaster Tools verification code (msvalidate.01)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bing: Option<Cow<'static, str>>,

    /// Pinterest verification code (p:domain_verify)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinterest: Option<Cow<'static, str>>,

    /// Facebook Domain Verification code (facebook-domain-verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<Cow<'static, str>>,

    /// Other verification meta tags not covered by the specific fields.
    /// Each entry is a tuple `(name, content)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<(Cow<'static, str>, Cow<'static, str>)>>,
}

impl Verification {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(512);

        meta_tags!(html,
            ("meta", "name", "google-site-verification", self.google.as_deref()),
            ("meta", "name", "yandex-verification", self.yandex.as_deref()),
            ("meta", "name", "msvalidate.01", self.bing.as_deref()),
            ("meta", "name", "p:domain_verify", self.pinterest.as_deref()),
            ("meta", "name", "facebook-domain-verification", self.facebook.as_deref()),
        );

        if let Some(other_list) = &self.other {
            for (name, content) in other_list {
                html.push_str("<meta name=\"");
                html.push_str(name.as_ref());
                html.push_str("\" content=\"");
                html.push_str(content.as_ref());
                html.push_str("\" />\n");
            }
        }

        html
    }

    /// Collects flat tag descriptors for all verification meta tags.
    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        // Standard verification tags
        if let Some(google) = &self.google {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "google-site-verification".to_string() },
                    TagProp { key: "content".to_string(), value: google.to_string() },
                ],
            });
        }
        if let Some(yandex) = &self.yandex {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "yandex-verification".to_string() },
                    TagProp { key: "content".to_string(), value: yandex.to_string() },
                ],
            });
        }
        if let Some(bing) = &self.bing {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "msvalidate.01".to_string() },
                    TagProp { key: "content".to_string(), value: bing.to_string() },
                ],
            });
        }
        if let Some(pinterest) = &self.pinterest {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "p:domain_verify".to_string() },
                    TagProp { key: "content".to_string(), value: pinterest.to_string() },
                ],
            });
        }
        if let Some(facebook) = &self.facebook {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "name".to_string(), value: "facebook-domain-verification".to_string() },
                    TagProp { key: "content".to_string(), value: facebook.to_string() },
                ],
            });
        }

        // Custom verification tags
        if let Some(other_list) = &self.other {
            for (name, content) in other_list {
                tags.push(TagDescriptor {
                    r#type: "meta".to_string(),
                    content: None,
                    props: vec![
                        TagProp { key: "name".to_string(), value: name.to_string() },
                        TagProp { key: "content".to_string(), value: content.to_string() },
                    ],
                });
            }
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            google: self.google.clone().or_else(|| parent.google.clone()),
            yandex: self.yandex.clone().or_else(|| parent.yandex.clone()),
            bing: self.bing.clone().or_else(|| parent.bing.clone()),
            pinterest: self.pinterest.clone().or_else(|| parent.pinterest.clone()),
            facebook: self.facebook.clone().or_else(|| parent.facebook.clone()),
            other: self.other.clone().or_else(|| parent.other.clone()),
        }
    }
}