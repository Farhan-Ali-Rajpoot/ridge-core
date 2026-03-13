use std::{
    borrow::Cow,
};

use rscx::{component, html, props};


#[props]
#[derive(Clone)]
pub struct LayoutProps {
    pub children: Option<Cow<'static, str>>,
    pub class: Option<Cow<'static, str>>,
}

#[props]
#[derive(Default, Debug, Clone)]
pub struct RootLayoutProps {
    #[builder(setter(into))]
    pub children: Cow<'static, str>,
    #[builder(setter(into))]
    pub class: Option<Cow<'static, str>>,
    #[builder(setter(into))]
    pub metadata: Option<Cow<'static, str>>,
}

#[component]
pub fn DefaultDocumentLayout(props: RootLayoutProps) -> String {
    html! {
        <!DOCTYPE html>
        <html lang="en" class={props.class.as_deref().unwrap_or("")}>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                {props.metadata.as_deref().unwrap_or("")}
            </head>
            <body>
                {props.children}
            </body>
        </html>
    }
}