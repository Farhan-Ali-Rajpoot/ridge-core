use serde::{Serialize};




#[derive(Clone, Debug, Default, Serialize)]
pub struct OwnContext<'a> {
    pub current_stack: Option<&'a str>,
    pub version: Option<&'a str>,
    // etc.
}

impl<'a> OwnContext<'a> {
    pub fn parse(header: &'a str) -> Self {
        let mut ctx = Self::default();

        for pair in header.split(';') {
            if let Some((key, value)) = pair.split_once('=') {
                match key.trim() {
                    "current_stack" => ctx.current_stack = Some(value.trim()),
                    "version"       => ctx.version       = Some(value.trim()),
                    _ => {} 
                }
            }
        }

        ctx
    }
}