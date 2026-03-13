use super::{TagDescriptor, TagProp};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Serialize)]
pub struct Icon {
    pub rel: Cow<'static, str>,
    pub url: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Cow<'static, str>>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            rel: Cow::Borrowed(""),
            url: Cow::Borrowed(""),
            sizes: None,
            type_: None,
        }
    }
}

impl Icon {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(256);
        html.push_str("<link");
        html.push_str(" rel=\"");
        html.push_str(self.rel.as_ref());
        html.push('"');
        html.push_str(" href=\"");
        html.push_str(self.url.as_ref());
        html.push('"');
        if let Some(sizes) = &self.sizes {
            html.push_str(" sizes=\"");
            html.push_str(sizes.as_ref());
            html.push('"');
        }
        if let Some(type_) = &self.type_ {
            html.push_str(" type=\"");
            html.push_str(type_.as_ref());
            html.push('"');
        }
        html.push_str(" />\n");
        html
    }

    pub fn collect_tags(&self, tags: &mut Vec<TagDescriptor>) {
        let mut props = vec![
            TagProp { key: "rel".to_string(), value: self.rel.to_string() },
            TagProp { key: "href".to_string(), value: self.url.to_string() },
        ];
        if let Some(sizes) = &self.sizes {
            props.push(TagProp { key: "sizes".to_string(), value: sizes.to_string() });
        }
        if let Some(type_) = &self.type_ {
            props.push(TagProp { key: "type".to_string(), value: type_.to_string() });
        }
        tags.push(TagDescriptor {
            r#type: "link".to_string(),
            content: None,
            props,
        });
    }

    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            rel: self.rel.clone(),
            url: self.url.clone(),
            sizes: self.sizes.clone().or_else(|| parent.sizes.clone()),
            type_: self.type_.clone().or_else(|| parent.type_.clone()),
        }
    }
}