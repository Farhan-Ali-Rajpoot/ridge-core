use super::core::{RouteMetadata};
use super::types::custom_tag::{MetaTag, LinkTag};



impl RouteMetadata {
    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            // Core
            title: self.title.clone().or_else(|| parent.title.clone()),
            description: self.description.clone().or_else(|| parent.description.clone()),
            keywords: self.keywords.clone().or_else(|| parent.keywords.clone()),
            authors: self.authors.clone().or_else(|| parent.authors.clone()),
            creator: self.creator.clone().or_else(|| parent.creator.clone()),
            publisher: self.publisher.clone().or_else(|| parent.publisher.clone()),
            category: self.category.clone().or_else(|| parent.category.clone()),
            classification: self.classification.clone().or_else(|| parent.classification.clone()),
            referrer: self.referrer.clone().or(parent.referrer.clone()),
        
            // Robots
            robots: match (&self.robots, &parent.robots) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
            googlebot: match (&self.googlebot, &parent.googlebot) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Canonical & Alternates
            alternates: match (&self.alternates, &parent.alternates) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Open Graph
            open_graph: match (&self.open_graph, &parent.open_graph) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Twitter
            twitter: match (&self.twitter, &parent.twitter) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Icons
            icons: self.icons.clone().or_else(|| parent.icons.clone()),
        
            // PWA
            manifest: self.manifest.clone().or_else(|| parent.manifest.clone()),
            theme_color: self.theme_color.clone().or_else(|| parent.theme_color.clone()),
            color_scheme: self.color_scheme.clone().or_else(|| parent.color_scheme.clone()),
        
            // Apple Web App
            apple_web_app: match (&self.apple_web_app, &parent.apple_web_app) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // App Links
            app_links: match (&self.app_links, &parent.app_links) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Verification
            verification: match (&self.verification, &parent.verification) {
                (Some(child), Some(parent)) => Some(child.inherit_from(parent)),
                (Some(child), None) => Some(child.clone()),
                (None, Some(parent)) => Some(parent.clone()),
                (None, None) => None,
            },
        
            // Structured Data
            json_ld: self.json_ld.clone().or_else(|| parent.json_ld.clone()),
        
            // Custom Meta / Link
            custom_meta: {
                let mut merged = Vec::new();
                let parent_map: std::collections::HashMap<_, &MetaTag> = parent.custom_meta
                    .iter()
                    .filter_map(|tag| tag.key().map(|k| (k, tag)))
                    .collect();
            
                for child in &self.custom_meta {
                    if let Some(key) = child.key() {
                        if let Some(parent_tag) = parent_map.get(&key) {
                            merged.push(child.inherit_from(parent_tag));
                        } else {
                            merged.push(child.clone());
                        }
                    } else {
                        merged.push(child.clone());
                    }
                }
            
                for parent_tag in &parent.custom_meta {
                    if let Some(key) = parent_tag.key() {
                        if !self.custom_meta.iter().any(|child| child.key().as_ref() == Some(&key)) {
                            merged.push(parent_tag.clone());
                        }
                    } else if !self.custom_meta.contains(parent_tag) {
                        merged.push(parent_tag.clone());
                    }
                }
                merged
            },
        
            custom_links: {
                let mut merged = Vec::new();
                let parent_map: std::collections::HashMap<_, &LinkTag> = parent.custom_links
                    .iter()
                    .filter_map(|tag| tag.key().map(|k| (k, tag)))
                    .collect();
            
                for child in &self.custom_links {
                    if let Some(key) = child.key() {
                        if let Some(parent_tag) = parent_map.get(&key) {
                            merged.push(child.inherit_from(parent_tag));
                        } else {
                            merged.push(child.clone());
                        }
                    } else {
                        merged.push(child.clone());
                    }
                }
            
                for parent_tag in &parent.custom_links {
                    if let Some(key) = parent_tag.key() {
                        if !self.custom_links.iter().any(|child| child.key().as_ref() == Some(&key)) {
                            merged.push(parent_tag.clone());
                        }
                    } else if !self.custom_links.contains(parent_tag) {
                        merged.push(parent_tag.clone());
                    }
                }
                merged
            },
        
            // App Behavior
            stack_hint: self.stack_hint.clone().or_else(|| parent.stack_hint.clone()),
            requires_auth: self.requires_auth,
            skip_layouts: self.skip_layouts,
        
            // i18n variants
            i18n_variants: {
                let mut merged = parent.i18n_variants.clone();
                for (key, child_val) in &self.i18n_variants {
                    if let Some(parent_val) = merged.get(key) {
                        // Recursively inherit the variant
                        merged.insert(key.clone(), child_val.inherit_from(parent_val));
                    } else {
                        merged.insert(key.clone(), child_val.clone());
                    }
                }
                merged
            },

            // mode variants
            mode_variants: {
                let mut merged = parent.mode_variants.clone();
                for (key, child_val) in &self.mode_variants {
                    if let Some(parent_val) = merged.get(key) {
                        merged.insert(key.clone(), child_val.inherit_from(parent_val));
                    } else {
                        merged.insert(key.clone(), child_val.clone());
                    }
                }
                merged
            },

            // extensions – simple child override (shallow)
            extensions: {
                let mut merged = parent.extensions.clone();
                for (key, val) in &self.extensions {
                    merged.insert(key.clone(), val.clone());
                }
                merged
            },
        }
    }
}