
use omnyx::{
    builder::{AppBuilder, Config, Renderer}, collections::LinearMap, include_dir::{self, Dir, include_dir}, request::Request, router::{LayoutProps, RenderedParallelRoute, Router}, rscx::html
};

static PUBLIC_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn main() {

    let root_layout = async move |req: Request, props: LayoutProps| {
        omnyx::rscx::html! {
            <!DOCTYPE html>
                <html lang="en">
                    <head>
                        { &req.metadata().render_html() }
                        <meta charset="utf-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                        <link rel="stylesheet" href="/public/dist/styles/styles.css" />
                        <script type="module" src="/public/dist/js/bundle.js" defer=true></script>
                    </head>
                    <body class="min-h-screen w-full bg-[var(--color-bg-base)] text-[var(--color-text-base)] font-haffer-montreal antialiased">
                        { props.children }
                    </body>
                </html>
        } 
    };

    let router = home_router();

    let config = Config {
        address: "127.0.0.1:3000".into(),
        embedded_public_dir: Some(&PUBLIC_DIR),
    };

    let renderer = Renderer::new()
        .root_layout_handler(root_layout);

    let app = AppBuilder::new()
        .with_config(config)
        .with_router(router)
        .with_renderer(renderer)
        .build()
        .unwrap();

    app.run();
}

#[derive(serde::Deserialize)]
pub struct Payload {
    query: Option<String>,
}

pub fn home_router() -> Router {
    Router::new()
        .layout("home", |layout| {
            layout
                .loader_handler(|| async move {
                    "Loading..."
                })
                .handler(|props: LayoutProps| async move {
                    let d = RenderedParallelRoute {
                        html: "None".into(),
                        params: LinearMap::new()
                    };          
                    let sidebar = props.parallel_routes.get("@sidebar").unwrap_or(&d);         

                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                    html! {
                        <div>
                            <div>Navbar</div>
                            { props.children }
                            { &sidebar.html }
                            <div>Footer</div>
                        </div>
                    }

                })
                .error_handler(async move || {
                    "Error occured in layout"
                })
                .parallel_route("@sidebar", |r| {
                    r
                    .page("/[[...slug]]", |page| {
                        page
                        .handler(|| async move { html! { <div>Parallel Route</div> } })
                        .loader_handler(|| async move { html!{ "Loading Sidebar"} })
                        .error_handler(|| async move { Err::<&str, &str>("z") })
                        .children(|r| {
                            r
                            .page("/user", |page| {
                                page
                                .handler(|| async move {  tokio::time::sleep(std::time::Duration::from_secs(3)).await;   html! { User_Sidebar }  })
                                .loader_handler(|| async move { html! { "Loading User_Sidebar "}})
                                .error_handler(|| async move { html! { "Error User_Sidebar"}})
                            })
                        })
                    })
                })
                .children(|router| {
                    router
                        .page("/", |page| {
                            page
                            .method("GET", |req: Request| async move {
                                if let Ok(payload) = req.body::<Payload>().await {
                                    println!("{}", payload.query.unwrap_or("None".into()));
                                } else {
                                    println!("Body read failed");
                                }
                            })
                            .children(|r| {
                                r
                                .page("/user", |page|  {
                                    page
                                    .method("GET", || async move {
                                        tokio::time::sleep(std::time::Duration::from_secs(3)).await; Err::<&str, &str>("E")
                                    })
                                    .loader_handler(|| async move {
                                        "Loading User"
                                    })
                                    .error_handler(|| async move {
                                        "ErroredUser----"
                                    })
                                })
                                .page("/admin", |page| {
                                    page
                                    .method("GET", || async move {
                                        html! { Admin }
                                    })
                                })
                            })
                        })
                })
        })
}