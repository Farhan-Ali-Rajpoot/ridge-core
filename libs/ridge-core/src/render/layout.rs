use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::core::router::metadata::RouteMetadata;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutProps {
    pub children: String,                  
    pub slots: HashMap<String, String>,      
    pub metadata: RouteMetadata,            
    pub params: HashMap<String, String>,    
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            children: Cow::Borrowed(""),
            class: None,
            id: None,
            attrs: HashMap::new(),
            extensions: HashMap::new(),
        }
    }
}

impl LayoutProps {
    pub fn new(children: impl Into<Cow<'static, str>>) -> Self {
        Self {
            children: children.into(),
            ..Default::default()
        }
    }

    pub fn with_class(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn with_id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.insert(key.into(), value.into());
        self
    }

    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
}


pub type LayoutFuture = Pin<Box<dyn Future<Output = String> + Send>>;

pub trait LayoutFn: Send + Sync + 'static {
    fn render(&self, props: LayoutProps) -> LayoutFuture;
}


impl<F, Fut> LayoutFn for F
where
    F: Fn(LayoutProps) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send + 'static,
{
    fn render(&self, props: LayoutProps) -> LayoutFuture {
        Box::pin((self)(props))
    }
}



pub type SharedLayout = Arc<dyn LayoutFn>;

pub fn layout<L>(layout: L) -> SharedLayout
where
    L: LayoutFn,
{
    Arc::new(layout)
}