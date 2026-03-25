use std::collections::HashMap;
use std::sync::Arc;
use omnyx::{
    OmnyxBuilder,
    core::{
        router::{
            Response ,RouteMetadata, MatchitMatcher, RouteNode, Path,
            response::IntoResponse,
        },
    },
};
use omnyx::prelude::*;
use axum::http::Method;
use async_trait::async_trait;
use rscx::{html};


fn main () {
   let mut router = MatchitMatcher::new();
   let ctx = RequestContext::default();

   let settings_page = RouteNode::Page {
        path: Path::from_str("/[...settings]"),
        handlers: HashMap::from([(Method::GET, Arc::new(Home) as Arc<dyn PageComponent>)]),
        error_handlers: HashMap::new(),
        metadata: RouteMetadata::default(),
        children: vec![],
        loaders: vec![],
        middlewares: vec![],
        extensions: HashMap::new(),
    };

    let user_profile_page = RouteNode::Page {
        path: Path::from_str("/[slug]"),
        handlers: HashMap::from([(Method::GET, Arc::new(Home) as Arc<dyn PageComponent>)]),
        error_handlers: HashMap::new(),
        metadata: RouteMetadata::default(),
        children: vec![settings_page], // Nesting the settings page here
        loaders: vec![],
        middlewares: vec![],
        extensions: HashMap::new(),
    };

    let user_root = RouteNode::Page {
        path: Path::from_str("/user"),
        handlers: HashMap::from([(Method::GET, Arc::new(Home) as Arc<dyn PageComponent>)]),
        error_handlers: HashMap::new(),
        metadata: RouteMetadata::default(),
        children: vec![user_profile_page],
        loaders: vec![],
        middlewares: vec![],
        extensions: HashMap::new(),
    };


    router.register(&user_root);

    if let Some(matched) = router.match_route("/user/farhan/settings", Method::GET) {
        println!("{:#?}", matched)
    }else {
        println!("Not Found")
    }
}


pub async fn Home(ctx: RequestContext) -> Option<String> {
    Some(html! { <div> {ctx.params.get("id")} </div>})
}



