#[derive(Clone, Debug, Default)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub original: String,
}

#[derive(Clone, Debug)]
pub enum PathSegment {
    Static(String),
    Dynamic(String),           // [slug]
    CatchAll(String),          // [...slug]
    OptionalCatchAll(String),  // [[...slug]]
}

impl Path {
    /// Creates a structured Path from a string like "/user/[id]/settings"
    pub fn from_str(path: &str) -> Self {
        let trimmed = path.trim().trim_matches('/');
        
        if trimmed.is_empty() {
            return Self {
                segments: vec![PathSegment::Static("".to_string())],
                original: "/".to_string(),
            };
        }

        let segments = trimmed
            .split('/')
            .map(PathSegment::parse_one)
            .collect();

        Self { 
            segments,
            original: format!("/{}", trimmed),
        }
    }

    /// Compiles the entire path for the Matchit router
    /// Example: "/user/[id]" -> "/user/:id"
    pub fn to_matchit_pattern(&self) -> String {
        let mut pattern = String::new();
        for segment in &self.segments {
            let part = segment.to_matchit_single();
            if part.is_empty() && self.segments.len() == 1 {
                return "/".to_string();
            }
            pattern.push('/');
            pattern.push_str(&part);
        }
        pattern
    }
}

impl PathSegment {
    fn parse_one(segment: &str) -> Self {
        let s = segment.trim();

        if s.starts_with("[[...") && s.ends_with("]]") {
            Self::OptionalCatchAll(s[5..s.len() - 2].to_string())
        } else if s.starts_with("[...") && s.ends_with("]") {
            Self::CatchAll(s[4..s.len() - 1].to_string())
        } else if s.starts_with('[') && s.ends_with(']') {
            Self::Dynamic(s[1..s.len() - 1].to_string())
        } else {
            Self::Static(s.to_string())
        }
    }

    fn to_matchit_single(&self) -> String {
        match self {
            Self::Static(s) => s.trim_matches('/').to_string(),
            Self::Dynamic(name) => format!("{{{}}}", name),
            Self::CatchAll(name) | Self::OptionalCatchAll(name) => format!("{{*{}}}", name),
        }
    }
}