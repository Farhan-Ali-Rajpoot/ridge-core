use thiserror::Error;



#[derive(Error, Debug)]
pub enum RouteError {
    #[error("Path '{0}' not found in the route tree")]
    NotFound(String),

    #[error("Segment conflict: '{0}' is already registered")]
    Conflict(String),

    #[error("Missing required metadata or extension: {0}")]
    MissingData(String),

    #[error("Missing Handler")]
    MissingHandler(String),

    #[error("Unexpected route error: {0}")]
    Other(String),
}


