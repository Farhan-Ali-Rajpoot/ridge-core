use super::core::{RouteMetadata};


impl RouteMetadata {
    pub fn render_html(&self) -> String {
        let mut html = String::with_capacity(2048);

        // Title
        if let Some(title) = &self.title {
            html.push_str(&format!("<title>{}</title>\n", title.as_ref()));
        }

        // Core meta tags
        meta_tags!(html,
            ("meta", "name", "description", self.description.as_deref()),
            ("meta", "name", "creator", self.creator.as_deref()),
            ("meta", "name", "publisher", self.publisher.as_deref()),
            ("meta", "name", "category", self.category.as_deref()),
            ("meta", "name", "classification", self.classification.as_deref()),
        );

        // Keywords
        if let Some(keywords) = &self.keywords {
            let kw_str: Vec<&str> = keywords.iter().map(|k| k.as_ref()).collect();
            html.push_str(&format!("<meta name=\"keywords\" content=\"{}\" />\n", kw_str.join(", ")));
        }

        // Authors
        if let Some(authors) = &self.authors {
            for author in authors {
                html.push_str(&author.render_html());
            }
        }

        // Referrer policy
        if let Some(ref rp) = self.referrer {
            html.push_str(&format!("<meta name=\"referrer\" content=\"{}\" />\n", rp.as_str()));
        }

        // Robots
        if let Some(robots) = &self.robots {
            html.push_str(&robots.render_html());
        }
        if let Some(googlebot) = &self.googlebot {
            html.push_str(&googlebot.render_html().replace("robots", "googlebot"));
        }

        // Canonical & Alternates
        if let Some(alt) = &self.alternates {
            html.push_str(&alt.render_html());
        }

        // Open Graph
        if let Some(og) = &self.open_graph {
            html.push_str(&og.render_html());
        }

        // Twitter
        if let Some(tw) = &self.twitter {
            html.push_str(&tw.render_html());
        }

        // Icons
        if let Some(icons) = &self.icons {
            for icon in icons {
                html.push_str(&icon.render_html());
            }
        }

        // PWA
        if let Some(manifest) = &self.manifest {
            html.push_str(&format!("<link rel=\"manifest\" href=\"{}\" />\n", manifest.as_ref()));
        }
        if let Some(theme) = &self.theme_color {
            html.push_str(&format!("<meta name=\"theme-color\" content=\"{}\" />\n", theme.as_ref()));
        }
        if let Some(scheme) = &self.color_scheme {
            html.push_str(&format!("<meta name=\"color-scheme\" content=\"{}\" />\n", scheme.as_ref()));
        }

        // Apple Web App
        if let Some(apple) = &self.apple_web_app {
            html.push_str(&apple.render_html());
        }

        // App Links
        if let Some(app) = &self.app_links {
            html.push_str(&app.render_html());
        }

        // Verification
        if let Some(ver) = &self.verification {
            html.push_str(&ver.render_html());
        }

        // Structured Data (JSON-LD)
        if let Some(json) = &self.json_ld {
            html.push_str("<script type=\"application/ld+json\">");
            html.push_str(json.as_ref());
            html.push_str("</script>\n");
        }

        // Custom meta tags
        for meta in &self.custom_meta {
            html.push_str(&meta.render_html());
        }

        // Custom link tags
        for link in &self.custom_links {
            html.push_str(&link.render_html());
        }

        // Stack hint (app behavior)
        if let Some(hint) = &self.stack_hint {
            html.push_str(&format!("<meta name=\"stack-hint\" content=\"{}\" />\n", hint.as_ref()));
        }

        for (key, value) in &self.i18n_variants {

        }
        // requires_auth and skip_layouts are not directly output in HTML;
        // they are used by the application logic.

        html
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

