






#[macro_export]
macro_rules! meta_tags {
    ($buffer:expr, $( 
        // Pattern 1: Meta tags (tag, attribute, name, value)
        // Usage: ("meta", "name", "description", self.description)
        ($tag:literal, $attr:literal, $name:expr, $value:expr) 
    ),* $(,)?) => {
        $(
            if let Some(v) = $value {
                $buffer.push_str("<");
                $buffer.push_str($tag);
                $buffer.push_str(" ");
                $buffer.push_str($attr);
                $buffer.push_str("=\"");
                $buffer.push_str($name);
                $buffer.push_str("\" content=\"");
                $buffer.push_str(v);
                $buffer.push_str("\" />\n");
            }
        )*
    };

    // Extension: Overload to handle Link tags specifically if needed
    // Usage: meta_tags!(link: buffer, ("link", "rel", "canonical", self.canonical))
    (link: $buffer:expr, $( ($tag:literal, $attr:literal, $name:expr, $value:expr) ),* $(,)?) => {
        $(
            if let Some(v) = $value {
                $buffer.push_str("<");
                $buffer.push_str($tag);
                $buffer.push_str(" ");
                $buffer.push_str($attr);
                $buffer.push_str("=\"");
                $buffer.push_str($name);
                $buffer.push_str("\" href=\"");
                $buffer.push_str(v);
                $buffer.push_str("\" />\n");
            }
        )*
    };
}