use thiserror::Error;

pub use crate::error::route_error::RouteError;

#[derive(Error, Debug)]
pub enum RidgeError {
    #[error(transparent)]
    Route(#[from] RouteError),
}


