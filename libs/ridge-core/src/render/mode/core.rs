use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Eq, Hash,  Serialize, Deserialize)]
pub enum RenderMode {
    FullHtml,
    Fragment,
    Streaming,
    RscPayload,
    Offline,
    Custom(String),
}

