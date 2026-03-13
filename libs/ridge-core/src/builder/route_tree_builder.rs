use std::borrow::Cow;
use async_trait::async_trait;


use crate::error::route_error::RouteError;
use crate::core::router::RouteTree;
use crate::core::router::RouteNode;


#[derive(Clone, Debug)]
pub enum RouteTreeBuilderInfo {
    CodeDefined {
        code_blocks_count: usize,
        built_at: std::time::SystemTime,
        description: Option<Cow<'static, str>>,
    },
}


#[async_trait]
pub trait RouteTreeBuilder: Send + Sync + 'static {
    async fn build(&self) -> Result<RouteTree, RouteError>;

}

