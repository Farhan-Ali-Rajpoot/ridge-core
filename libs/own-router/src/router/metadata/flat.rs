use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TagProp {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagDescriptor {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub props: Vec<TagProp>,
}