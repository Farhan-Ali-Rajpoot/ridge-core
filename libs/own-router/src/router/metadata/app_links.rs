use super::{TagDescriptor, TagProp}; 
use serde::Serialize;
use std::borrow::Cow;


#[derive(Clone, Debug, Default, Serialize)]
pub struct IosAppLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_id: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<Cow<'static, str>>,
}

impl IosAppLink {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        meta_tags!(html,
            ("meta", "property", "al:ios:url", self.url.as_deref()),
            ("meta", "property", "al:ios:app_store_id", self.app_store_id.as_deref()),
            ("meta", "property", "al:ios:app_name", self.app_name.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        let mut props = Vec::new();
        if let Some(url) = &self.url {
            props.push(TagProp { key: "property".to_string(), value: "al:ios:url".to_string() });
            props.push(TagProp { key: "content".to_string(), value: url.to_string() });
        }
        if let Some(app_store_id) = &self.app_store_id {
            props.push(TagProp { key: "property".to_string(), value: "al:ios:app_store_id".to_string() });
            props.push(TagProp { key: "content".to_string(), value: app_store_id.to_string() });
        }
        if let Some(app_name) = &self.app_name {
            props.push(TagProp { key: "property".to_string(), value: "al:ios:app_name".to_string() });
            props.push(TagProp { key: "content".to_string(), value: app_name.to_string() });
        }
        // Group by same tag? Each property is a separate meta tag.
        // We need to push one TagDescriptor per meta tag.
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:ios:url".to_string() },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
        if let Some(app_store_id) = &self.app_store_id {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:ios:app_store_id".to_string() },
                    TagProp { key: "content".to_string(), value: app_store_id.to_string() },
                ],
            });
        }
        if let Some(app_name) = &self.app_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:ios:app_name".to_string() },
                    TagProp { key: "content".to_string(), value: app_name.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone().or_else(|| parent.url.clone()),
            app_store_id: self.app_store_id.clone().or_else(|| parent.app_store_id.clone()),
            app_name: self.app_name.clone().or_else(|| parent.app_name.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AndroidAppLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<Cow<'static, str>>,
}

impl AndroidAppLink {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        meta_tags!(html,
            ("meta", "property", "al:android:url", self.url.as_deref()),
            ("meta", "property", "al:android:package", self.package.as_deref()),
            ("meta", "property", "al:android:class", self.class.as_deref()),
            ("meta", "property", "al:android:app_name", self.app_name.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:android:url".to_string() },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
        if let Some(package) = &self.package {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:android:package".to_string() },
                    TagProp { key: "content".to_string(), value: package.to_string() },
                ],
            });
        }
        if let Some(class) = &self.class {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:android:class".to_string() },
                    TagProp { key: "content".to_string(), value: class.to_string() },
                ],
            });
        }
        if let Some(app_name) = &self.app_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:android:app_name".to_string() },
                    TagProp { key: "content".to_string(), value: app_name.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone().or_else(|| parent.url.clone()),
            package: self.package.clone().or_else(|| parent.package.clone()),
            class: self.class.clone().or_else(|| parent.class.clone()),
            app_name: self.app_name.clone().or_else(|| parent.app_name.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WindowsAppLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<Cow<'static, str>>,
}

impl WindowsAppLink {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        meta_tags!(html,
            ("meta", "property", "al:windows:url", self.url.as_deref()),
            ("meta", "property", "al:windows:app_id", self.app_id.as_deref()),
            ("meta", "property", "al:windows:app_name", self.app_name.as_deref()),
        );
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:windows:url".to_string() },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
        if let Some(app_id) = &self.app_id {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:windows:app_id".to_string() },
                    TagProp { key: "content".to_string(), value: app_id.to_string() },
                ],
            });
        }
        if let Some(app_name) = &self.app_name {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:windows:app_name".to_string() },
                    TagProp { key: "content".to_string(), value: app_name.to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone().or_else(|| parent.url.clone()),
            app_id: self.app_id.clone().or_else(|| parent.app_id.clone()),
            app_name: self.app_name.clone().or_else(|| parent.app_name.clone()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WebAppLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_fallback: Option<bool>,
}

impl WebAppLink {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(128);

        if let Some(url) = &self.url {
            html.push_str(&format!("<meta property=\"al:web:url\" content=\"{}\" />\n", url.as_ref()));
        }

        if let Some(fb) = self.should_fallback {
            let value = if fb { "true" } else { "false" };
            html.push_str(&format!("<meta property=\"al:web:should_fallback\" content=\"{}\" />\n", value));
        }

        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(url) = &self.url {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:web:url".to_string() },
                    TagProp { key: "content".to_string(), value: url.to_string() },
                ],
            });
        }
        if let Some(fb) = self.should_fallback {
            tags.push(TagDescriptor {
                r#type: "meta".to_string(),
                content: None,
                props: vec![
                    TagProp { key: "property".to_string(), value: "al:web:should_fallback".to_string() },
                    TagProp { key: "content".to_string(), value: (if fb { "true" } else { "false" }).to_string() },
                ],
            });
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            url: self.url.clone().or_else(|| parent.url.clone()),
            should_fallback: self.should_fallback.or(parent.should_fallback),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AppLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<IosAppLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<AndroidAppLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<WindowsAppLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebAppLink>,
}

impl AppLinks {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(512);

        if let Some(ios) = &self.ios {
            html.push_str(&ios.render_html());
        }
        if let Some(android) = &self.android {
            html.push_str(&android.render_html());
        }
        if let Some(windows) = &self.windows {
            html.push_str(&windows.render_html());
        }
        if let Some(web) = &self.web {
            html.push_str(&web.render_html());
        }

        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        if let Some(ios) = &self.ios {
            ios.collect_tags(tags);
        }
        if let Some(android) = &self.android {
            android.collect_tags(tags);
        }
        if let Some(windows) = &self.windows {
            windows.collect_tags(tags);
        }
        if let Some(web) = &self.web {
            web.collect_tags(tags);
        }
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            ios: match (&self.ios, &parent.ios) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            android: match (&self.android, &parent.android) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            windows: match (&self.windows, &parent.windows) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            web: match (&self.web, &parent.web) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        }
    }
}