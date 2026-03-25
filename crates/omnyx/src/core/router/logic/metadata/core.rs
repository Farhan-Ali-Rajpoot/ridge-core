use super::types::{
    AppleWebApp, Icon, Author, TwitterCard, OpenGraph, Robots, Alternates, ReferrerPolicy, MaxImagePreview,
    MediaAlternate, TypeAlternate, AppLinks, Verification, MetaTag, LinkTag, LanguageAlternate,

    TagDescriptor, TagProp,
};
use const_format::formatcp;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::collections::HashMap;
use serde_json::Value;


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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


    #[serde(default)]
    pub i18n_variants: HashMap<Cow<'static, str>, RouteMetadata>, 
    #[serde(default)]
    pub mode_variants: HashMap<Cow<'static, str>, RouteMetadata>, 
    #[serde(default)]
    pub extensions: HashMap<Cow<'static, str>, Value>, 
}



