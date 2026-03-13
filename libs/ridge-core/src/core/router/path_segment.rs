









#[derive(Clone, Debug)]
pub enum PathSegment {
    Static(String),
    Dynamic(String),           // [slug]
    CatchAll(String),          // [...slug]
    OptionalCatchAll(String),  // [[...slug]]
}

impl PathSegment {
    /// Is this a static folder name?
    pub fn is_static(&self) -> bool {
        matches!(self, PathSegment::Static(_))
    }

    /// Is this [param] ?
    pub fn is_dynamic(&self) -> bool {
        matches!(self, PathSegment::Dynamic(_))
    }

    /// Is this [...param] or [[...param]] ?
    pub fn is_catch_all(&self) -> bool {
        matches!(self, PathSegment::CatchAll(_) | PathSegment::OptionalCatchAll(_))
    }

    /// Is this the optional version [[...param]] ?
    pub fn is_optional_catch_all(&self) -> bool {
        matches!(self, PathSegment::OptionalCatchAll(_))
    }

    pub fn param_name(&self) -> Option<&str> {
        match self {
            PathSegment::Dynamic(name) => Some(name),
            PathSegment::CatchAll(name) => Some(name),
            PathSegment::OptionalCatchAll(name) => Some(name),
            PathSegment::Static(_) => None,
        }
    }

    pub fn to_pattern(&self) -> String {
        match self {
            PathSegment::Static(s) => s.clone(),
            PathSegment::Dynamic(name) => format!("[{}]", name),
            PathSegment::CatchAll(name) => format!("[...{}]", name),
            PathSegment::OptionalCatchAll(name) => format!("[[...{}]]", name),
        }
    }

    pub fn to_matchit_pattern(&self) -> String {
        match self {
            PathSegment::Static(s) => s.clone(),
            PathSegment::Dynamic(name) => format!(":{}", name),
            PathSegment::CatchAll(name) => format!("*{}", name),
            PathSegment::OptionalCatchAll(name) => format!("*{}", name),
        }
    }

    pub fn parse_segment(segment: &str) -> PathSegment {
        let s = segment.trim();

        if s.is_empty() {
            return PathSegment::Static("".to_string());
        }

        if s.starts_with("[[...") && s.ends_with("]]") {
            let name = &s[5..s.len() - 2];
            if name.is_empty() || name.contains('[') || name.contains(']') {
                panic!("Invalid optional catch-all name in segment: {}", segment);
            }
            return PathSegment::OptionalCatchAll(name.to_string());
        }

        if s.starts_with("[...") && s.ends_with("]") {
            let name = &s[4..s.len() - 1];
            if name.is_empty() || name.contains('[') || name.contains(']') {
                panic!("Invalid catch-all name in segment: {}", segment);
            }
            return PathSegment::CatchAll(name.to_string());
        }

        if s.starts_with('[') && s.ends_with(']') {
            let name = &s[1..s.len() - 1];
            if name.is_empty() || name.contains('[') || name.contains(']') {
                panic!("Invalid dynamic parameter name in segment: {}", segment);
            }
            if name.starts_with("...") {
                panic!("Use [...name] syntax for catch-all, not [..name]");
            }
            return PathSegment::Dynamic(name.to_string());
        }

        if s.contains('[') || s.contains(']') {
            panic!("Static segment cannot contain [ or ]: {}", segment);
        }
        PathSegment::Static(s.to_string())
    }
}